use {
    super::{ArbitrageError, Result},
    crate::{
        arbitrage::{orca_util, raydium_util},
        processor::ProcessorError,
        BlockState,
    },
    anyhow::bail,
    common::{
        message::SequenceId, AmountType, LiquidityGroupCode, LiquidityPool, LiquidityPoolState, Side,
        Token, TokenAccount, TokenCode, TokenDigitsType, ONE_BP_DECIMAL,
    },
    log::debug,
    raydium_amm_interface::AmmInfo,
    solana_sdk::{
        clock::{Slot, UnixTimestamp},
        pubkey::Pubkey,
        signature::Signature,
    },
    spl_token::state::Account,
    std::{cell::RefCell, collections::HashMap, rc::Rc},
    whirlpool_interface::WhirlpoolAccount,
};

impl From<ArbitrageError> for ProcessorError {
    fn from(error: ArbitrageError) -> Self {
        ProcessorError::ProcessingError {
            msg: error.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct ArbitrageCache {
    block_state: BlockState,
    lp_states: HashMap<Pubkey, Rc<RefCell<LiquidityPoolState>>>,
    liguidity_groups: HashMap<LiquidityGroupCode, Vec<Rc<RefCell<LiquidityPoolState>>>>,
    arb_sequencer: RefCell<SequenceId>,
    token_accounts: HashMap<TokenCode, TokenAccount>,
    pub(crate) token_account_balances: HashMap<Pubkey, TokenDigitsType>,
}

impl ArbitrageCache {
    pub fn new() -> Self {
        Self {
            block_state: BlockState {
                slot: 0,
                block_time: 0,
                block_height: 0,
            },
            lp_states: HashMap::new(),
            liguidity_groups: HashMap::new(),
            arb_sequencer: RefCell::new(SequenceId::new()),
            token_accounts: HashMap::new(),
            token_account_balances: HashMap::new(),
        }
    }

    pub fn liquidity_groups(
        &self,
    ) -> &HashMap<LiquidityGroupCode, Vec<Rc<RefCell<LiquidityPoolState>>>> {
        &self.liguidity_groups
    }

    pub fn add_liquidity_pool(&mut self, liquidity_pool: LiquidityPool) {
        let accounts = LiquidityPool::accounts(&liquidity_pool);

        let liquidity_group = LiquidityPool::liquidity_group(&liquidity_pool);
        let lp_states = self
            .liguidity_groups
            .entry(liquidity_group.clone())
            .or_insert(Vec::new());

        let lp_state = Rc::new(RefCell::new(LiquidityPoolState::new(liquidity_pool)));
        lp_states.push(lp_state.clone());

        for account in accounts.keys() {
            self.lp_states.insert(*account, lp_state.clone());
        }
    }

    pub fn get_lp_state(&self, address: &Pubkey) -> Option<&Rc<RefCell<LiquidityPoolState>>> {
        self.lp_states.get(address)
    }

    pub fn next_arb_sequence_id(&self) -> SequenceId {
        self.arb_sequencer.borrow_mut().increment_and_get()
    }

    pub fn block_state(&self) -> &BlockState {
        &self.block_state
    }

    pub fn update_block_state(
        &mut self,
        slot: Slot,
        block_time: Option<UnixTimestamp>,
        block_height: Option<u64>,
    ) {
        self.block_state = BlockState {
            slot,
            block_time: block_time.unwrap_or_default(),
            block_height: block_height.unwrap_or_default(),
        };
    }

    pub fn update_orca_whirlpool(
        &mut self,
        address: &Pubkey,
        wp_account: &WhirlpoolAccount,
        slot: Slot,
        txn_signature: Option<Signature>,
    ) -> Result<Option<LiquidityGroupCode>> {
        debug!(
            "Updating Orca Whirlpool account {}. {:?}",
            Pubkey::from(*address).to_string(),
            wp_account
        );

        if let Some(lp_state_ref) = self.get_lp_state(address) {
            let token_amounts = orca_util::calc_token_amounts(&wp_account.0).ok_or(
                ArbitrageError::TokenAmountsCalculationFailure {
                    msg: format!("{:?}", wp_account.0),
                },
            )?;
            let account_fee = orca_util::calc_fee(&wp_account.0);

            let mut lp_state = lp_state_ref.borrow_mut();
            // Confirm that recalc is required.
            if lp_state.base_token_digits == token_amounts.0
                && lp_state.quote_token_digits == token_amounts.1
                && (lp_state.fee_ratio - account_fee).abs() < ONE_BP_DECIMAL
            {
                return Ok(None);
            }

            lp_state.base_token_digits = token_amounts.0;
            lp_state.quote_token_digits = token_amounts.1;
            lp_state.fee_ratio = account_fee;
            lp_state.healthy = true;
            lp_state.latest_upd_slot = slot;
            lp_state.latest_upd_txn_sig = txn_signature;

            Ok(Some(lp_state.liquidity_group()))
        } else {
            bail!(ArbitrageError::UnrecognizedLiquidityPool { address: *address })
        }
    }

    pub fn update_raydium_amm_account(
        &mut self,
        address: &Pubkey,
        amm_info: &AmmInfo,
        slot: Slot,
        txn_signature: Option<Signature>,
    ) -> Result<Option<LiquidityGroupCode>> {
        debug!(
            "Updating Raydium Amm account {}. {:?}",
            Pubkey::from(*address).to_string(),
            amm_info
        );

        if let Some(lp_state_ref) = self.get_lp_state(address) {
            let amm_info_fee = raydium_util::calc_fee(amm_info);
            let healthy_flag = raydium_util::calc_healthy_flag(&amm_info).ok_or(
                ArbitrageError::UnrecognizedRaydiumAmmAccountUpdateStatus {
                    msg: format!("{:?}", amm_info),
                },
            )?;

            let mut lp_state = lp_state_ref.borrow_mut();
            if (lp_state.fee_ratio - amm_info_fee).abs() < ONE_BP_DECIMAL
                && lp_state.healthy == healthy_flag
            {
                return Ok(None);
            }

            // No price update is reguired since neither fee nor the healthy flag are participating in the price calculation.
            lp_state.fee_ratio = amm_info_fee;
            lp_state.healthy = healthy_flag;
            lp_state.latest_upd_slot = slot;
            lp_state.latest_upd_txn_sig = txn_signature;

            Ok(Some(lp_state.liquidity_group()))
        } else {
            bail!(ArbitrageError::UnrecognizedLiquidityPool { address: *address })
        }
    }

    pub fn update_raydium_amm_vault(
        &mut self,
        address: &Pubkey,
        account: &Account,
        side: Side,
        slot: Slot,
        txn_signature: Option<Signature>,
    ) -> Result<Option<LiquidityGroupCode>> {
        debug!(
            "Updating Raydium Amm {:?} Vault account {} in Arb cache. {:?}",
            side,
            Pubkey::from(*address).to_string(),
            account
        );

        if let Some(lp_state_ref) = self.get_lp_state(address) {
            let mut lp_state = lp_state_ref.borrow_mut();
            match side {
                Side::Base => {
                    if lp_state.base_token_digits != account.amount as TokenDigitsType {
                        lp_state.base_token_digits = account.amount as TokenDigitsType;
                        lp_state.latest_upd_slot = slot;
                        lp_state.latest_upd_txn_sig = txn_signature;
                        Ok(Some(lp_state.liquidity_group()))
                    } else {
                        Ok(None)
                    }
                }
                Side::Quote => {
                    if lp_state.quote_token_digits != account.amount as TokenDigitsType {
                        lp_state.quote_token_digits = account.amount as TokenDigitsType;
                        lp_state.latest_upd_slot = slot;
                        lp_state.latest_upd_txn_sig = txn_signature;
                        Ok(Some(lp_state.liquidity_group()))
                    } else {
                        Ok(None)
                    }
                }
            }
        } else {
            bail!(ArbitrageError::UnrecognizedRaydiumVoteAccount { address: *address })
        }
    }

    pub fn register_token_account(&mut self, token_account: TokenAccount) {
        self.token_accounts.insert(token_account.code(), token_account);
    }

    pub fn update_token_account_balance(
        &mut self,
        token_mint: Pubkey,
        new_balance_digits: TokenDigitsType,
    ) {
        self.token_account_balances.insert(token_mint, new_balance_digits);
    }

    pub fn get_available_token_account_balance(&self, token: &Token) -> Result<AmountType> {
        let token_account: &TokenAccount =
            self.token_accounts
                .get(&token.code())
                .ok_or(ArbitrageError::UnsupportedTokenCode {
                    code: token.code().to_string(),
                })?;

        let balance: AmountType = *self.token_account_balances.get(&token.mint()).unwrap_or(&0)
            as AmountType
            / 10u64.pow(token.decimals().into()) as AmountType;
        let adjusted_balance = balance - token_account.min_balance();

        if adjusted_balance > 0.0 {
            Ok(adjusted_balance)
        } else {
            bail!("Low balance ({})!", balance);
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::ArbitrageCache,
        common::{
            test_util::tests::{
                create_liquidity_pool, create_orca_whirlpool_account, create_raydium_amm_info,
                create_spl_account, MOCK_stSOL_TOKEN, MOCK_wSOL_TOKEN_ACCOUNT, MOCK_USDC_TOKEN,
                MOCK_USDC_TOKEN_ACCOUNT, MOCK_USDT_TOKEN_ACCOUNT,
            },
            AccountType, AmountType, LiquidityGroupCode, LiquidityPool, PriceType, Side,
            TokenDigitsType,
        },
        solana_sdk::{
            clock::{Slot, UnixTimestamp},
            signature::Signature,
        },
    };

    #[test]
    fn test_add_liquidity_pool() {
        let mut arb_cache = ArbitrageCache::new();
        assert_eq!(arb_cache.liguidity_groups.len(), 0);
        assert_eq!(arb_cache.lp_states.len(), 0);

        let mut accounts: usize = 0;
        for i in 1..10 {
            let lp = create_liquidity_pool(if i % 2 == 0 {
                AccountType::RaydiumAmmPoolAccount
            } else {
                AccountType::OrcaWhirlpoolAccount
            });

            accounts += match lp {
                LiquidityPool::OrcaWhirlpool { .. } => 1, // just lp address
                LiquidityPool::RaydiumAmm { .. } => 3,    // lp address + 2 vote accounts
            };

            arb_cache.add_liquidity_pool(lp);
            assert_eq!(arb_cache.liguidity_groups.len(), 1);
            assert_eq!(arb_cache.lp_states.len(), accounts);
        }
    }

    #[test]
    fn test_lp_state() {
        let lp: LiquidityPool = create_liquidity_pool(AccountType::RaydiumAmmPoolAccount);
        let lp_address = LiquidityPool::address(&lp).clone();
        //        let liquidity_grop = LiquidityPool::liquidity_group(&lp).clone();

        let mut arb_cache = ArbitrageCache::new();
        assert!(arb_cache.get_lp_state(&lp_address).is_none());
        assert!(arb_cache
            .liguidity_groups
            .get(&LiquidityGroupCode::SOL_USD)
            .is_none());

        arb_cache.add_liquidity_pool(lp.clone());

        const BASE_AMOUNT: TokenDigitsType = 1111;
        const QUOTE_AMOUNT: TokenDigitsType = 2222;
        const FEE: PriceType = 3.45;
        {
            let mut lp_state2 = arb_cache.get_lp_state(&lp_address).unwrap().borrow_mut();
            lp_state2.base_token_digits = BASE_AMOUNT;
            lp_state2.quote_token_digits = QUOTE_AMOUNT;
            lp_state2.fee_ratio = FEE;
        }

        let lp_state3 = arb_cache
            .liguidity_groups
            .get(&LiquidityGroupCode::SOL_USD)
            .unwrap()
            .iter()
            .find(|&lps| *LiquidityPool::address(&lps.borrow().liquidity_pool) == lp_address)
            .unwrap()
            .borrow();
        assert_eq!(lp_state3.base_token_digits, BASE_AMOUNT);
        assert_eq!(
            lp_state3.base_token_decimals(),
            LiquidityPool::base_token(&lp).decimals()
        );
        assert_eq!(lp_state3.quote_token_digits, QUOTE_AMOUNT);
        assert_eq!(
            lp_state3.quote_token_decimals(),
            LiquidityPool::quote_token(&lp).decimals()
        );
        assert_eq!(lp_state3.fee_ratio, FEE);
    }

    #[test]
    fn test_update_raydium_amm_account() {
        let lp = create_liquidity_pool(AccountType::RaydiumAmmPoolAccount);
        let lp_address = LiquidityPool::address(&lp).clone();
        let lg = LiquidityPool::liquidity_group(&lp);
        let slot = 123;
        let signature = Some(Signature::new_unique());

        let mut arb_cache = ArbitrageCache::new();
        arb_cache.add_liquidity_pool(lp.clone());
        let lp_state = arb_cache.get_lp_state(&lp_address).unwrap();
        assert_eq!(
            lp_state.borrow().base_token_decimals(),
            LiquidityPool::base_token(&lp).decimals()
        );
        assert_eq!(
            lp_state.borrow().quote_token_decimals(),
            LiquidityPool::quote_token(&lp).decimals()
        );
        assert!(lp_state.borrow().fee_ratio.is_nan());
        assert!(!lp_state.borrow().healthy);

        let amm_info = create_raydium_amm_info();
        let upd_result = arb_cache
            .update_raydium_amm_account(&lp_address, &amm_info, slot, signature)
            .unwrap();
        assert_eq!(upd_result, Some(lg));

        // Check in lp_states
        let lp_state = arb_cache.get_lp_state(&lp_address).unwrap();
        assert!(lp_state.borrow().fee_ratio > 0f64);
        assert!(lp_state.borrow().healthy);

        // Check in liquidity_groups
        let lg = arb_cache
            .liguidity_groups
            .get(&LiquidityGroupCode::SOL_USD)
            .unwrap();
        lg.iter()
            .find(|&lp_state| *LiquidityPool::address(&lp_state.borrow().liquidity_pool) == lp_address)
            .unwrap();
    }

    #[test]
    fn test_update_raydium_amm_vault() {
        let lp = create_liquidity_pool(AccountType::RaydiumAmmPoolAccount);
        let lp_address = LiquidityPool::address(&lp).clone();
        let lg = LiquidityPool::liquidity_group(&lp);
        const SLOT1: Slot = 123;
        const SLOT2: Slot = 456;
        let signature1 = Some(Signature::new_unique());
        let signature2 = Some(Signature::new_unique());

        let mut arb_cache = ArbitrageCache::new();
        arb_cache.add_liquidity_pool(lp);
        let lp_state = arb_cache.get_lp_state(&lp_address).unwrap();
        assert_eq!(lp_state.borrow().base_token_digits, 0);
        assert_eq!(lp_state.borrow().quote_token_digits, 0);
        assert!(lp_state.borrow().fee_ratio.is_nan());
        assert!(!lp_state.borrow().healthy);

        let base_account = create_spl_account(111);
        let quote_account = create_spl_account(222);
        let upd_result = arb_cache
            .update_raydium_amm_vault(&lp_address, &base_account, Side::Base, SLOT1, signature1)
            .unwrap();
        assert_eq!(upd_result, Some(lg));
        let upd_result = arb_cache
            .update_raydium_amm_vault(&lp_address, &quote_account, Side::Quote, SLOT2, signature2)
            .unwrap();
        assert_eq!(upd_result, Some(lg));

        // Check in lp_states
        let lp_state = arb_cache.get_lp_state(&lp_address).unwrap();
        assert_eq!(
            lp_state.borrow().base_token_digits,
            base_account.amount as TokenDigitsType
        );
        assert_eq!(
            lp_state.borrow().quote_token_digits,
            quote_account.amount as TokenDigitsType
        );
        assert_eq!(lp_state.borrow().latest_upd_slot, SLOT2);
        assert_eq!(lp_state.borrow().latest_upd_txn_sig, signature2);

        // Check liquidity groups
        let lg = arb_cache
            .liguidity_groups
            .get(&LiquidityGroupCode::SOL_USD)
            .unwrap();
        let lp_state = lg
            .iter()
            .find(|&lp_state| *LiquidityPool::address(&lp_state.borrow().liquidity_pool) == lp_address)
            .unwrap();
        assert_eq!(
            lp_state.borrow().base_token_digits,
            base_account.amount as TokenDigitsType
        );
        assert_eq!(
            lp_state.borrow().quote_token_digits,
            quote_account.amount as TokenDigitsType
        );
        assert_eq!(lp_state.borrow().latest_upd_slot, SLOT2);
        assert_eq!(lp_state.borrow().latest_upd_txn_sig, signature2);
    }

    #[test]
    fn test_update_orca_whirlpool() {
        let lp = create_liquidity_pool(AccountType::OrcaWhirlpoolAccount);
        let lp_address = LiquidityPool::address(&lp).clone();
        let lg = LiquidityPool::liquidity_group(&lp);
        const SLOT: Slot = 123;
        let signature = Some(Signature::new_unique());

        let mut arb_cache = ArbitrageCache::new();
        arb_cache.add_liquidity_pool(lp);
        let lp_state = arb_cache.get_lp_state(&lp_address).unwrap();
        assert_eq!(lp_state.borrow().base_token_digits, 0);
        assert_eq!(lp_state.borrow().quote_token_digits, 0);
        assert!(lp_state.borrow().fee_ratio.is_nan());

        let mut wp_account = create_orca_whirlpool_account();
        wp_account.0.liquidity = 93538458143942;
        wp_account.0.sqrt_price = 7933124927893604393;
        wp_account.0.tick_current_index = -16878;
        wp_account.0.tick_spacing = 2;

        let upd_result = arb_cache
            .update_orca_whirlpool(&lp_address, &wp_account, SLOT, signature)
            .unwrap();
        assert_eq!(upd_result, Some(lg));

        let lp_state = arb_cache.get_lp_state(&lp_address).unwrap();
        assert!(lp_state.borrow().base_token_digits > 0);
        assert!(lp_state.borrow().quote_token_digits > 0);
        assert!(lp_state.borrow().fee_ratio > 0f64);
        assert_eq!(lp_state.borrow().latest_upd_slot, SLOT);
        assert_eq!(lp_state.borrow().latest_upd_txn_sig, signature);
    }

    #[test]
    fn test_update_block_state() {
        let mut cache = ArbitrageCache::new();
        assert_eq!(cache.block_state.slot, 0);
        assert_eq!(cache.block_state.block_time, 0);
        assert_eq!(cache.block_state.block_height, 0);

        const SLOT: Slot = 123;
        const BLOCK_TIME: UnixTimestamp = 346;
        const BLOCK_HEIGHT: u64 = 789;
        cache.update_block_state(SLOT, Some(BLOCK_TIME), Some(BLOCK_HEIGHT));
        assert_eq!(cache.block_state.slot, SLOT);
        assert_eq!(cache.block_state.block_time, BLOCK_TIME);
        assert_eq!(cache.block_state.block_height, BLOCK_HEIGHT);
    }

    #[test]
    fn test_token_account_balances() {
        let mut cache = ArbitrageCache::new();
        cache.register_token_account(MOCK_wSOL_TOKEN_ACCOUNT.clone());
        cache.register_token_account(MOCK_USDC_TOKEN_ACCOUNT.clone());
        cache.register_token_account(MOCK_USDT_TOKEN_ACCOUNT.clone());

        // Token is not in the Wallet
        assert!(cache
            .get_available_token_account_balance(&MOCK_stSOL_TOKEN)
            .is_err());

        // Token balance is too low
        let min_balance: AmountType = cache
            .token_accounts
            .get(&MOCK_USDC_TOKEN.code())
            .unwrap()
            .min_balance();
        let mut new_balance: AmountType = min_balance - (min_balance * 0.01);
        let mut new_balance_digits: TokenDigitsType =
            (new_balance * 10f64.powi(MOCK_USDC_TOKEN.decimals().into())) as TokenDigitsType;
        cache.update_token_account_balance(*MOCK_USDC_TOKEN.mint(), new_balance_digits);
        assert!(cache
            .get_available_token_account_balance(&MOCK_USDC_TOKEN)
            .is_err());

        // Good balance
        new_balance = min_balance + (min_balance * 0.01);
        new_balance_digits =
            (new_balance * 10f64.powi(MOCK_USDC_TOKEN.decimals().into())) as TokenDigitsType;
        cache.update_token_account_balance(*MOCK_USDC_TOKEN.mint(), new_balance_digits);
        assert_eq!(
            cache
                .get_available_token_account_balance(&MOCK_USDC_TOKEN)
                .unwrap(),
            new_balance - min_balance
        );
    }
}
