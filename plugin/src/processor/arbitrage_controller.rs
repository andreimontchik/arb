use {
    crate::{
        arbitrage::{ArbitrageCache, ArbitrageExecutor},
        processor::{Processor, ProcessorError, Result},
        LastUpdateSlotCache, PluginMetricKey,
    },
    anyhow::bail,
    borsh::de::BorshDeserialize,
    common::{
        message::{AccountUpdateMessage, ArbitrageMessage, BlockUpdateMessage},
        metrics::MetricsCollector,
        read_from_file, AccountType, AmountType, LiquidityPool, LiquidityPoolState, Side, TokenAccount,
    },
    core::f64,
    log::{debug, error, info},
    raydium_amm_interface::AmmInfo,
    serde_derive::Deserialize,
    serde_json::Value,
    solana_sdk::{clock::Slot, program_pack::Pack, pubkey::Pubkey, signature::Signature},
    std::{cell::RefCell, collections::HashMap, rc::Rc, time::Instant},
    whirlpool_interface::WhirlpoolAccount,
};

#[derive(Deserialize)]
struct ArbitrageControllerConfig {
    // TODO: move to LG configuration
    total_arb_iterations: u16,
    min_base_swap_amount: AmountType,
    min_quote_margin: AmountType,
    executor: Value,
    metrics: Value,
}
impl ArbitrageControllerConfig {
    pub fn load(config_file_name: &str) -> Self {
        info!("Loading ArbitrageController config from ({}).", &config_file_name);
        let config_file_contents = read_from_file(config_file_name);
        serde_json::from_str(&config_file_contents).unwrap()
    }
}
#[derive(Debug)]
pub struct ArbitrageController<M: MetricsCollector, T: ArbitrageExecutor> {
    accounts: HashMap<Pubkey, AccountType>,
    last_account_update_cache: LastUpdateSlotCache<Pubkey>,
    arb_cache: ArbitrageCache,
    arb_executor: T,
    metrics_collector: M,

    // TODO: move to LG configuration
    total_arb_iterations: u16,
    min_base_swap_amount: AmountType,
    min_quote_margin: AmountType,
}

impl<M: MetricsCollector, T: ArbitrageExecutor> ArbitrageController<M, T> {
    fn process_orca_whirlpool_account_update(
        &mut self,
        slot: Slot,
        address: &Pubkey,
        data: &Vec<u8>,
        txn_signature: Option<Signature>,
    ) -> Result<()> {
        let whirlpool = WhirlpoolAccount::deserialize(&data).map_err(|err| {
            ProcessorError::DeserializationError {
                msg: format!(
                    "slot: {}, address: {}, data: ({:?}), error: {:?}",
                    slot,
                    Pubkey::from(*address).to_string(),
                    data,
                    err
                ),
            }
        })?;

        self.arb_cache
            .update_orca_whirlpool(address, &whirlpool, slot, txn_signature)?;

        Ok(())
    }

    fn process_raydium_amm_account_update(
        &mut self,
        slot: Slot,
        address: &Pubkey,
        data: &Vec<u8>,
        txn_signature: Option<Signature>,
    ) -> Result<()> {
        let mut buffer = &data[..];
        let amm_info: AmmInfo =
            AmmInfo::deserialize(&mut buffer).map_err(|err| ProcessorError::DeserializationError {
                msg: format!(
                    "slot: {}, address: {}, data: ({:?}), error: {:?}",
                    slot,
                    Pubkey::from(*address).to_string(),
                    data,
                    err
                ),
            })?;

        self.arb_cache
            .update_raydium_amm_account(address, &amm_info, slot, txn_signature)?;
        Ok(())
    }

    fn process_raydium_amm_vault_update(
        &mut self,
        slot: Slot,
        address: &Pubkey,
        side: Side,
        data: &Vec<u8>,
        txn_signature: Option<Signature>,
    ) -> Result<()> {
        let account = spl_token::state::Account::unpack(&data).map_err(|err| {
            ProcessorError::DeserializationError {
                msg: format!(
                    "slot: {}, address: {}, data: ({:?}), error: {:?}",
                    slot, address, data, err
                ),
            }
        })?;
        self.arb_cache
            .update_raydium_amm_vault(address, &account, side, slot, txn_signature)?;
        Ok(())
    }

    fn process_token_account_update(
        &mut self,
        slot: Slot,
        address: &Pubkey,
        data: &Vec<u8>,
    ) -> Result<()> {
        if !data.is_empty() {
            let account = spl_token::state::Account::unpack(&data).map_err(|err| {
                ProcessorError::DeserializationError {
                    msg: format!(
                        "slot: {}, address: {}, data: ({:?}), error: {:?}",
                        slot, address, data, err
                    ),
                }
            })?;

            info!(
                "Updating the Token Account ({}) balance to ({})",
                address, account.amount
            );

            self.arb_cache
                .update_token_account_balance(account.mint, account.amount);
            Ok(())
        } else {
            bail!("The Token Account update data is empty for ({})!", address);
        }
    }

    fn evaluate_for_arbitrage(&mut self) -> Vec<ArbitrageMessage> {
        let start = Instant::now();

        let mut result: Vec<ArbitrageMessage> = vec![];

        for (_lg, lp_states) in self.arb_cache.liquidity_groups() {
            if let (Some(min_price_lp_state), Some(max_price_lp_state)) =
                get_lp_states_with_min_max_qoute_amount(lp_states, self.min_base_swap_amount)
            {
                // Dereferencing to underlying and then passing it by reference.
                // TODO: benchmark to confirm that it is faster than passing Rc<RefCell<T>>
                let buy_lp_state = &*min_price_lp_state.borrow();
                let sell_lp_state = &*max_price_lp_state.borrow();

                // Check token account balances
                let base_token = sell_lp_state.base_token();
                let quote_token = buy_lp_state.quote_token();
                match (
                    &self.arb_cache.get_available_token_account_balance(base_token),
                    &self.arb_cache.get_available_token_account_balance(quote_token),
                ) {
                    (
                        Ok(available_base_token_account_balance),
                        Ok(available_quote_token_account_balance),
                    ) => {
                        if let Some(arbitrage) = self.calculate_arbitrage(
                            buy_lp_state,
                            sell_lp_state,
                            *available_base_token_account_balance,
                            *available_quote_token_account_balance,
                        ) {
                            result.push(arbitrage);
                        }
                    }
                    (Err(error), _) => error!(
                        "The base Token Account ({:?}) balance Error! {}",
                        buy_lp_state.base_token(),
                        error
                    ),
                    (_, Err(error)) => error!(
                        "The quote Token Account ({:?}) balance Error! {}",
                        buy_lp_state.quote_token(),
                        error
                    ),
                }
            }
        }

        self.metrics_collector
            .duration(&PluginMetricKey::ArbitrageEvaluateDuration, start.elapsed());
        result
    }

    fn calculate_arbitrage(
        &self,
        buy_lp_state: &LiquidityPoolState,
        sell_lp_state: &LiquidityPoolState,
        base_token_account_balance: AmountType,
        quote_token_account_balance: AmountType,
    ) -> Option<ArbitrageMessage> {
        let mut base_token_amount: AmountType = f64::NAN;
        let mut buy_quote_token_amount: AmountType = f64::NAN;
        let mut sell_quote_token_amount: AmountType = f64::NAN;
        let mut quote_token_amount_margin: AmountType = 0.0;

        let step = base_token_account_balance / self.total_arb_iterations as AmountType;
        for i in 1..self.total_arb_iterations {
            let base_amount = step * i as AmountType;
            // Make sure that there is enough tokens on the arb base token account
            if base_amount > base_token_account_balance {
                break;
            }

            let buy_amount: AmountType = buy_lp_state.calc_quote_token_amount_to_buy(base_amount);
            // Make sure that there is enough tokens on the arb quote token account
            if buy_amount > quote_token_account_balance {
                break;
            }

            let sell_amount: AmountType = sell_lp_state.calc_quote_token_amount_for_selling(base_amount);
            if !sell_amount.is_finite() {
                break;
            }

            let margin = sell_amount - buy_amount;
            if margin > quote_token_amount_margin {
                base_token_amount = base_amount;
                buy_quote_token_amount = buy_amount;
                sell_quote_token_amount = sell_amount;
                quote_token_amount_margin = margin;
            } else {
                // Margin is decreasing no need to iterate anymore.
                break;
            }
        }

        if !(base_token_amount.is_finite()
            && buy_quote_token_amount.is_finite()
            && sell_quote_token_amount.is_finite()
            // Make sure that the profit is not too small
            && quote_token_amount_margin > self.min_quote_margin)
        {
            return None;
        }

        // Adjust to min profit for slippage.
        let quote_token_amount_to_adjust = (quote_token_amount_margin - self.min_quote_margin) / 2.0;
        buy_quote_token_amount += quote_token_amount_to_adjust;
        sell_quote_token_amount -= quote_token_amount_to_adjust;
        // Make sure the buying quote token amount is not over the token account balance
        if buy_quote_token_amount > quote_token_account_balance {
            buy_quote_token_amount = quote_token_account_balance;
            sell_quote_token_amount = buy_quote_token_amount + self.min_quote_margin;
        }

        Some(ArbitrageMessage::new(
            self.arb_cache.next_arb_sequence_id(),
            self.arb_cache.block_state().slot,
            buy_lp_state,
            sell_lp_state,
            base_token_amount,
            buy_quote_token_amount,
            sell_quote_token_amount,
        ))
    }
}

impl<M: MetricsCollector, T: ArbitrageExecutor> Processor for ArbitrageController<M, T> {
    fn new(config_file_name: &str) -> Self {
        let config = ArbitrageControllerConfig::load(config_file_name);

        ArbitrageController {
            accounts: HashMap::new(),
            last_account_update_cache: LastUpdateSlotCache::new(),
            arb_cache: ArbitrageCache::new(),
            arb_executor: T::new(config.executor),
            metrics_collector: M::new(config.metrics),
            total_arb_iterations: config.total_arb_iterations,
            min_base_swap_amount: config.min_base_swap_amount,
            min_quote_margin: config.min_quote_margin,
        }
    }

    fn register_token_account(&mut self, msg: TokenAccount) -> Result<()> {
        self.accounts.insert(*msg.address(), AccountType::TokenAccount);
        self.arb_cache.register_token_account(msg);
        Ok(())
    }

    fn update_liquidity_pool(&mut self, msg: LiquidityPool) -> Result<()> {
        self.accounts.extend(LiquidityPool::accounts(&msg));
        self.arb_cache.add_liquidity_pool(msg);
        Ok(())
    }

    fn update_block(&mut self, msg: BlockUpdateMessage) -> Result<()> {
        if msg.slot > self.arb_cache.block_state().slot {
            self.arb_cache
                .update_block_state(msg.slot, msg.block_time, msg.block_height);
            for arbitrage in self.evaluate_for_arbitrage() {
                let start = Instant::now();
                self.arb_executor.execute(&arbitrage).map(|_| ())?;
                self.metrics_collector
                    .duration(&PluginMetricKey::ArbitrageProcessDuration, start.elapsed());
            }
        }
        Ok(())
    }

    fn update_account(&mut self, msg: AccountUpdateMessage) -> Result<()> {
        if !self
            .last_account_update_cache
            .is_new_update(msg.slot, &msg.address)
        {
            debug!(
                "Ignoring an obsolete AccountUpdate message. Slot: {}, Address: {:?}",
                msg.slot,
                Pubkey::from(msg.address).to_string()
            );
            return Ok(());
        }

        let start = Instant::now();

        debug!(
            "Processing Account Update. Slot: {}, Address: {:?}",
            msg.slot,
            Pubkey::from(msg.address).to_string()
        );

        match self.accounts.get(&msg.address) {
            Some(account_type) => match account_type {
                AccountType::OrcaWhirlpoolAccount => self.process_orca_whirlpool_account_update(
                    msg.slot,
                    &msg.address,
                    &msg.data,
                    msg.txn_signature,
                ),
                AccountType::RaydiumAmmPoolAccount => self.process_raydium_amm_account_update(
                    msg.slot,
                    &msg.address,
                    &msg.data,
                    msg.txn_signature,
                ),
                AccountType::RaydiumAmmPoolVaultForBaseToken => self.process_raydium_amm_vault_update(
                    msg.slot,
                    &msg.address,
                    Side::Base,
                    &msg.data,
                    msg.txn_signature,
                ),
                AccountType::RaydiumAmmPoolVaultForQuoteToken => self.process_raydium_amm_vault_update(
                    msg.slot,
                    &msg.address,
                    Side::Quote,
                    &msg.data,
                    msg.txn_signature,
                ),
                AccountType::TokenAccount => {
                    self.process_token_account_update(msg.slot, &msg.address, &msg.data)
                }
                _ => unimplemented!(),
            },
            None => bail!(ProcessorError::UnrecognizedAccount { address: msg.address }),
        }?;

        self.last_account_update_cache.update(msg.slot, msg.address);

        self.metrics_collector
            .duration(&PluginMetricKey::AccountUpdateDuration, start.elapsed());

        Ok(())
    }
}

fn get_lp_states_with_min_max_qoute_amount(
    lp_states: &Vec<Rc<RefCell<LiquidityPoolState>>>,
    min_base_amount: AmountType,
) -> (
    std::option::Option<Rc<RefCell<LiquidityPoolState>>>,
    std::option::Option<Rc<RefCell<LiquidityPoolState>>>,
) {
    let mut min_lp_state: Option<Rc<RefCell<LiquidityPoolState>>> = None;
    let mut min_lp_state_quote_amount_to_buy: AmountType = 0.0;
    let mut max_lp_state: Option<Rc<RefCell<LiquidityPoolState>>> = None;
    let mut max_lp_state_quote_amount_for_selling: AmountType = 0.0;
    for lp_state_ref in lp_states {
        let lp_state = lp_state_ref.borrow();
        // Make sure that the state is legit.
        if lp_state.is_computable() {
            let lp_state_quote_amount_to_buy: AmountType =
                lp_state.calc_quote_token_amount_to_buy(min_base_amount);
            if min_lp_state.is_none() || lp_state_quote_amount_to_buy < min_lp_state_quote_amount_to_buy
            {
                min_lp_state = Some(lp_state_ref.clone());
                min_lp_state_quote_amount_to_buy = lp_state_quote_amount_to_buy;
            }

            let lp_state_quote_amount_for_selling =
                lp_state.calc_quote_token_amount_for_selling(min_base_amount);
            if max_lp_state.is_none()
                || lp_state_quote_amount_for_selling > max_lp_state_quote_amount_for_selling
            {
                max_lp_state = Some(lp_state_ref.clone());
                max_lp_state_quote_amount_for_selling = lp_state_quote_amount_for_selling;
            }
        }
    }

    if min_lp_state_quote_amount_to_buy < max_lp_state_quote_amount_for_selling {
        (min_lp_state, max_lp_state)
    } else {
        (None, None)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        borsh::BorshSerialize,
        common::{
            message::ArbitrageMessage,
            metrics::NoopMetricsCollector,
            test_util::tests::{
                create_liquidity_pool, create_orca_whirlpool_account, create_orca_whirlpool_lp,
                create_raydium_amm_info, create_raydium_amm_lp, create_spl_account,
                MOCK_wSOL_TOKEN_ACCOUNT, MOCK_USDC_TOKEN_ACCOUNT, MOCK_USDT_TOKEN_ACCOUNT,
            },
            DecimalPercentageType, TokenDigitsType, ONE_BP_DECIMAL,
        },
        solana_sdk::{pubkey::Pubkey, signature::Signature},
        spl_token::state::Account,
    };

    struct MockArbitrageExecutor {
        was_executed: bool,
    }

    impl ArbitrageExecutor for MockArbitrageExecutor {
        fn new(_: Value) -> Self {
            Self { was_executed: false }
        }

        fn execute(&mut self, _arbitrage: &ArbitrageMessage) -> Result<()> {
            self.was_executed = true;
            Ok(())
        }
    }

    const MOCK_TOTAL_ARB_ITERATIONS: u16 = 10;
    const MOCK_MIN_BASE_SWAP_AMOUNT: DecimalPercentageType = 0.1;
    const MOCK_MIN_QUOTE_MARGIN: AmountType = 1.0;
    const MOCK_BASE_TOKEN_BALANCE: AmountType = 10.0;
    const MOCK_QUOTE_TOKEN_BALANCE: AmountType = 1000.0;

    #[test]
    fn test_add_account() {
        let mut controller = ArbitrageController {
            accounts: HashMap::new(),
            last_account_update_cache: LastUpdateSlotCache::new(),
            arb_cache: ArbitrageCache::new(),
            arb_executor: MockArbitrageExecutor { was_executed: false },
            metrics_collector: NoopMetricsCollector {},
            total_arb_iterations: MOCK_TOTAL_ARB_ITERATIONS,
            min_base_swap_amount: MOCK_MIN_BASE_SWAP_AMOUNT,
            min_quote_margin: MOCK_MIN_QUOTE_MARGIN,
        };

        assert!(controller
            .update_liquidity_pool(create_liquidity_pool(AccountType::OrcaWhirlpoolAccount))
            .is_ok());
    }

    #[test]
    fn test_process_unsupported_account() {
        let mut controller = ArbitrageController {
            accounts: HashMap::new(),
            last_account_update_cache: LastUpdateSlotCache::new(),
            arb_cache: ArbitrageCache::new(),
            arb_executor: MockArbitrageExecutor { was_executed: false },
            metrics_collector: NoopMetricsCollector {},
            total_arb_iterations: MOCK_TOTAL_ARB_ITERATIONS,
            min_base_swap_amount: MOCK_MIN_BASE_SWAP_AMOUNT,
            min_quote_margin: MOCK_MIN_QUOTE_MARGIN,
        };

        assert!(controller
            .update_account(AccountUpdateMessage {
                slot: 1,
                address: Pubkey::new_unique(),
                data: vec![0; 256],
                txn_signature: Some(Signature::new_unique()),
            })
            .is_err())
    }

    #[test]
    fn test_process_orca_whirlpool_account_update() {
        let lp = create_liquidity_pool(AccountType::OrcaWhirlpoolAccount);
        let lp_address = *LiquidityPool::address(&lp);

        let whirlpool_account = create_orca_whirlpool_account();
        let mut data: Vec<u8> = Vec::new();
        assert!(whirlpool_account.serialize(&mut data).is_ok());

        let mut controller = ArbitrageController {
            accounts: HashMap::new(),
            last_account_update_cache: LastUpdateSlotCache::new(),
            arb_cache: ArbitrageCache::new(),
            arb_executor: MockArbitrageExecutor { was_executed: false },
            metrics_collector: NoopMetricsCollector {},
            total_arb_iterations: MOCK_TOTAL_ARB_ITERATIONS,
            min_base_swap_amount: MOCK_MIN_BASE_SWAP_AMOUNT,
            min_quote_margin: MOCK_MIN_QUOTE_MARGIN,
        };

        controller.update_liquidity_pool(lp).unwrap();
        controller
            .process_orca_whirlpool_account_update(1, &lp_address, &data, Some(Signature::new_unique()))
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_fail_to_process_orca_whirlpool_account_update() {
        let mut controller = ArbitrageController {
            accounts: HashMap::new(),
            last_account_update_cache: LastUpdateSlotCache::new(),
            arb_cache: ArbitrageCache::new(),
            arb_executor: MockArbitrageExecutor { was_executed: false },
            metrics_collector: NoopMetricsCollector {},
            total_arb_iterations: MOCK_TOTAL_ARB_ITERATIONS,
            min_base_swap_amount: MOCK_MIN_BASE_SWAP_AMOUNT,
            min_quote_margin: MOCK_MIN_QUOTE_MARGIN,
        };

        controller
            .process_orca_whirlpool_account_update(1, &Pubkey::new_unique(), &vec![0; 32], None)
            .unwrap();
    }

    #[test]
    fn test_process_raydium_amm_account_update() {
        let lp = create_liquidity_pool(AccountType::RaydiumAmmPoolAccount);
        let lp_address = *LiquidityPool::address(&lp);

        let mut controller = ArbitrageController {
            accounts: HashMap::new(),
            last_account_update_cache: LastUpdateSlotCache::new(),
            arb_cache: ArbitrageCache::new(),
            arb_executor: MockArbitrageExecutor { was_executed: false },
            metrics_collector: NoopMetricsCollector {},
            total_arb_iterations: MOCK_TOTAL_ARB_ITERATIONS,
            min_base_swap_amount: MOCK_MIN_BASE_SWAP_AMOUNT,
            min_quote_margin: MOCK_MIN_QUOTE_MARGIN,
        };

        controller.update_liquidity_pool(lp).unwrap();

        let amm = create_raydium_amm_info();
        let mut data: Vec<u8> = Vec::new();
        assert!(amm.serialize(&mut data).is_ok());

        controller
            .process_raydium_amm_account_update(1, &lp_address, &data, Some(Signature::new_unique()))
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_fail_to_process_raydium_amm_account_update() {
        let mut controller = ArbitrageController {
            accounts: HashMap::new(),
            last_account_update_cache: LastUpdateSlotCache::new(),
            arb_cache: ArbitrageCache::new(),
            arb_executor: MockArbitrageExecutor { was_executed: false },
            metrics_collector: NoopMetricsCollector {},
            total_arb_iterations: MOCK_TOTAL_ARB_ITERATIONS,
            min_base_swap_amount: MOCK_MIN_BASE_SWAP_AMOUNT,
            min_quote_margin: MOCK_MIN_QUOTE_MARGIN,
        };

        controller
            .process_raydium_amm_account_update(
                1,
                &Pubkey::new_unique(),
                &vec![0; 32],
                Some(Signature::new_unique()),
            )
            .unwrap();
    }

    #[test]
    fn test_process_token_account_update() {
        let mut controller = ArbitrageController {
            accounts: HashMap::new(),
            last_account_update_cache: LastUpdateSlotCache::new(),
            arb_cache: ArbitrageCache::new(),
            arb_executor: MockArbitrageExecutor { was_executed: false },
            metrics_collector: NoopMetricsCollector {},
            total_arb_iterations: MOCK_TOTAL_ARB_ITERATIONS,
            min_base_swap_amount: MOCK_MIN_BASE_SWAP_AMOUNT,
            min_quote_margin: MOCK_MIN_QUOTE_MARGIN,
        };
        controller
            .register_token_account(MOCK_wSOL_TOKEN_ACCOUNT.clone())
            .unwrap();

        let account_balance_digits: TokenDigitsType = 123456;

        let spl_token_account = create_spl_account(account_balance_digits);
        assert!(controller
            .arb_cache
            .token_account_balances
            .get(&spl_token_account.mint)
            .is_none());

        let mut data: Vec<u8> = vec![0; Account::LEN];
        assert!(Account::pack(spl_token_account, &mut data).is_ok());

        controller
            .process_token_account_update(1, MOCK_wSOL_TOKEN_ACCOUNT.address(), &data)
            .unwrap();

        assert_eq!(
            controller
                .arb_cache
                .token_account_balances
                .get(&spl_token_account.mint)
                .unwrap(),
            &account_balance_digits
        );

        // No token account data
        assert!(controller
            .process_token_account_update(1, MOCK_wSOL_TOKEN_ACCOUNT.address(), &vec![])
            .is_err());
    }

    #[test]
    fn test_process_raydium_amm_vault_update() {
        let lp = create_liquidity_pool(AccountType::RaydiumAmmPoolAccount);
        let lp_address = *LiquidityPool::address(&lp);

        let mut controller = ArbitrageController {
            accounts: HashMap::new(),
            last_account_update_cache: LastUpdateSlotCache::new(),
            arb_cache: ArbitrageCache::new(),
            arb_executor: MockArbitrageExecutor { was_executed: false },
            metrics_collector: NoopMetricsCollector {},
            total_arb_iterations: MOCK_TOTAL_ARB_ITERATIONS,
            min_base_swap_amount: MOCK_MIN_BASE_SWAP_AMOUNT,
            min_quote_margin: MOCK_MIN_QUOTE_MARGIN,
        };

        controller.update_liquidity_pool(lp).unwrap();

        let vault = create_spl_account(1);
        let mut data: Vec<u8> = vec![0; Account::LEN];
        assert!(Account::pack(vault, &mut data).is_ok());

        controller
            .process_raydium_amm_vault_update(
                1,
                &lp_address,
                Side::Base,
                &data,
                Some(Signature::new_unique()),
            )
            .unwrap();

        controller
            .process_raydium_amm_vault_update(
                1,
                &lp_address,
                Side::Quote,
                &data,
                Some(Signature::new_unique()),
            )
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_fail_to_process_raydium_amm_vault_update() {
        let mut controller = ArbitrageController {
            accounts: HashMap::new(),
            last_account_update_cache: LastUpdateSlotCache::new(),
            arb_cache: ArbitrageCache::new(),
            arb_executor: MockArbitrageExecutor { was_executed: false },
            metrics_collector: NoopMetricsCollector {},
            total_arb_iterations: MOCK_TOTAL_ARB_ITERATIONS,
            min_base_swap_amount: MOCK_MIN_BASE_SWAP_AMOUNT,
            min_quote_margin: MOCK_MIN_QUOTE_MARGIN,
        };

        controller
            .process_raydium_amm_vault_update(
                1,
                &Pubkey::new_unique(),
                Side::Base,
                &vec![0; 32],
                Some(Signature::new_unique()),
            )
            .unwrap();
    }

    #[test]
    fn test_update_block() {
        let mut controller = ArbitrageController {
            accounts: HashMap::new(),
            last_account_update_cache: LastUpdateSlotCache::new(),
            arb_cache: ArbitrageCache::new(),
            arb_executor: MockArbitrageExecutor { was_executed: false },
            metrics_collector: NoopMetricsCollector {},
            total_arb_iterations: MOCK_TOTAL_ARB_ITERATIONS,
            min_base_swap_amount: MOCK_MIN_BASE_SWAP_AMOUNT,
            min_quote_margin: MOCK_MIN_QUOTE_MARGIN,
        };

        const SLOT1: Slot = 111;
        const SLOT2: Slot = 222;

        let msg = BlockUpdateMessage {
            slot: SLOT2,
            block_time: None,
            block_height: None,
        };
        controller.update_block(msg).unwrap();
        assert_eq!(controller.arb_cache.block_state().slot, SLOT2);

        // Old slot, no update
        let msg = BlockUpdateMessage {
            slot: SLOT1,
            block_time: None,
            block_height: None,
        };
        controller.update_block(msg).unwrap();
        assert_eq!(controller.arb_cache.block_state().slot, SLOT2);
    }

    #[test]
    fn test_get_min_max_lp_states() {
        const PRICE_SPREAD_THRESHOLD: DecimalPercentageType = 1.0;

        let mut lg: Vec<Rc<RefCell<LiquidityPoolState>>> = Vec::new();

        // Empty liquidity group
        let (min_lps, max_lps) = get_lp_states_with_min_max_qoute_amount(&lg, PRICE_SPREAD_THRESHOLD);
        assert!(min_lps.is_none());
        assert!(max_lps.is_none());

        // One entry, but it is not available
        let lps1 = Rc::new(RefCell::new(LiquidityPoolState::new(create_liquidity_pool(
            AccountType::RaydiumAmmPoolAccount,
        ))));
        lg.push(lps1.clone());

        let (min_lps, max_lps) = get_lp_states_with_min_max_qoute_amount(&lg, PRICE_SPREAD_THRESHOLD);
        assert!(min_lps.is_none());
        assert!(max_lps.is_none());

        // One available entry, but it is not computable
        {
            lps1.borrow_mut().healthy = false;
        }

        let (min_lps, max_lps) = get_lp_states_with_min_max_qoute_amount(&lg, PRICE_SPREAD_THRESHOLD);
        assert!(min_lps.is_none());
        assert!(max_lps.is_none());

        // One available and computable entry
        {
            let mut lps_ref = lps1.borrow_mut();
            lps_ref.healthy = true;
            lps_ref.base_token_digits = 10 * 10u64.pow(lps_ref.base_token_decimals() as u32);
            lps_ref.quote_token_digits = 10_000 * 10u64.pow(lps_ref.quote_token_decimals() as u32);
            lps_ref.fee_ratio = 0.001;
            lps_ref.latest_upd_txn_sig = Some(Signature::new_unique());
        }

        let (min_lps, max_lps) = get_lp_states_with_min_max_qoute_amount(&lg, PRICE_SPREAD_THRESHOLD);
        assert!(min_lps.is_none());
        assert!(max_lps.is_none());

        // Two entries, but the second one is not available
        let lps2 = Rc::new(RefCell::new(LiquidityPoolState::new(create_liquidity_pool(
            AccountType::RaydiumAmmPoolAccount,
        ))));
        lg.push(lps2.clone());
        let (min_lps, max_lps) = get_lp_states_with_min_max_qoute_amount(&lg, PRICE_SPREAD_THRESHOLD);
        assert!(min_lps.is_none());
        assert!(max_lps.is_none());

        // Two available and computable entries
        {
            let mut lps_ref = lps2.borrow_mut();
            lps_ref.healthy = true;
            lps_ref.base_token_digits = 11 * 10u64.pow(lps_ref.base_token_decimals() as u32);
            lps_ref.quote_token_digits = 9_000 * 10u64.pow(lps_ref.quote_token_decimals() as u32);
            lps_ref.fee_ratio = 0.001;
            lps_ref.latest_upd_txn_sig = Some(Signature::new_unique());
        }
        let (min_lps, max_lps) = get_lp_states_with_min_max_qoute_amount(&lg, PRICE_SPREAD_THRESHOLD);
        assert_eq!(min_lps.unwrap(), lps2);
        assert_eq!(max_lps.unwrap(), lps1);

        // Min LP does not have enough base tokens
        {
            let mut lps2_ref = lps2.borrow_mut();
            lps2_ref.base_token_digits = (lps2_ref.min_base_token_balance()
                * 10u64.pow(lps2_ref.base_token_decimals() as u32) as f64
                - 1.0) as u64;
        }
        let (min_lps, max_lps) = get_lp_states_with_min_max_qoute_amount(&lg, PRICE_SPREAD_THRESHOLD);
        assert!(min_lps.is_none());
        assert!(max_lps.is_none());

        // max LPs does not have enough quite tokens
        {
            let mut lps_ref = lps2.borrow_mut();
            lps_ref.base_token_digits = 11 * 10u64.pow(lps_ref.base_token_decimals() as u32);
        }
        let (min_lps, max_lps) = get_lp_states_with_min_max_qoute_amount(&lg, PRICE_SPREAD_THRESHOLD);
        assert_eq!(min_lps.unwrap(), lps2);
        assert_eq!(max_lps.unwrap(), lps1);
        {
            let mut lps_ref = lps1.borrow_mut();
            lps_ref.quote_token_digits = (lps_ref.min_quote_token_balance()
                * 10u64.pow(lps_ref.quote_token_decimals() as u32) as f64
                - 1.0) as u64;
        }
        let (min_lps, max_lps) = get_lp_states_with_min_max_qoute_amount(&lg, PRICE_SPREAD_THRESHOLD);
        assert!(min_lps.is_none());
        assert!(max_lps.is_none());
    }

    #[test]
    fn test_get_min_max_lp_states_base_amount() {
        let lps1 = Rc::new(RefCell::new(LiquidityPoolState::new(create_liquidity_pool(
            AccountType::RaydiumAmmPoolAccount,
        ))));
        {
            let mut lps_ref = lps1.borrow_mut();
            lps_ref.base_token_digits = 11 * 10u64.pow(lps_ref.base_token_decimals() as u32);
            lps_ref.quote_token_digits = 9_000 * 10u64.pow(lps_ref.quote_token_decimals() as u32);
            lps_ref.fee_ratio = 0.001;
            lps_ref.latest_upd_txn_sig = Some(Signature::new_unique());
            lps_ref.healthy = true;
        }

        let lps2 = Rc::new(RefCell::new(LiquidityPoolState::new(create_liquidity_pool(
            AccountType::RaydiumAmmPoolAccount,
        ))));
        {
            let mut lps_ref = lps2.borrow_mut();
            lps_ref.base_token_digits = 12 * 10u64.pow(lps_ref.base_token_decimals() as u32);
            lps_ref.quote_token_digits = 8_000 * 10u64.pow(lps_ref.quote_token_decimals() as u32);
            lps_ref.fee_ratio = 0.001;
            lps_ref.latest_upd_txn_sig = Some(Signature::new_unique());
            lps_ref.healthy = true;
        }

        let lps3 = Rc::new(RefCell::new(LiquidityPoolState::new(create_liquidity_pool(
            AccountType::RaydiumAmmPoolAccount,
        ))));
        {
            let mut lps_ref = lps3.borrow_mut();
            lps_ref.base_token_digits = 13 * 10u64.pow(lps_ref.base_token_decimals() as u32);
            lps_ref.quote_token_digits = 7_000 * 10u64.pow(lps_ref.quote_token_decimals() as u32);
            lps_ref.fee_ratio = 0.001;
            lps_ref.latest_upd_txn_sig = Some(Signature::new_unique());
            lps_ref.healthy = true;
        }

        let lps4 = Rc::new(RefCell::new(LiquidityPoolState::new(create_liquidity_pool(
            AccountType::RaydiumAmmPoolAccount,
        ))));
        {
            let mut lps_ref = lps4.borrow_mut();
            lps_ref.base_token_digits = 14 * 10u64.pow(lps_ref.base_token_decimals() as u32);
            lps_ref.quote_token_digits = 6_000 * 10u64.pow(lps_ref.quote_token_decimals() as u32);
            lps_ref.fee_ratio = 0.001;
            lps_ref.latest_upd_txn_sig = Some(Signature::new_unique());
            lps_ref.healthy = true;
        }

        let mut lg: Vec<Rc<RefCell<LiquidityPoolState>>> = Vec::new();
        lg.push(lps3.clone());
        lg.push(lps1.clone());
        lg.push(lps4.clone());
        lg.push(lps2.clone());

        let mut min_base_amount = 1.0;
        let (min_lps, max_lps) = get_lp_states_with_min_max_qoute_amount(&lg, min_base_amount);
        assert_eq!(min_lps.unwrap(), lps4);
        assert_eq!(max_lps.unwrap(), lps1);

        // Min base amount is too high for some LPs.
        min_base_amount = 11.0;
        let (min_lps, max_lps) = get_lp_states_with_min_max_qoute_amount(&lg, min_base_amount);
        assert!(min_lps.is_none());
        assert!(max_lps.is_none());
    }

    #[test]
    fn test_evaluate_for_arbitrage() {
        let mut controller = ArbitrageController {
            accounts: HashMap::new(),
            last_account_update_cache: LastUpdateSlotCache::new(),
            arb_cache: ArbitrageCache::new(),
            arb_executor: MockArbitrageExecutor { was_executed: false },
            metrics_collector: NoopMetricsCollector {},
            total_arb_iterations: MOCK_TOTAL_ARB_ITERATIONS,
            min_base_swap_amount: MOCK_MIN_BASE_SWAP_AMOUNT,
            min_quote_margin: MOCK_MIN_QUOTE_MARGIN,
        };
        controller
            .register_token_account(MOCK_wSOL_TOKEN_ACCOUNT.clone())
            .unwrap();
        controller
            .register_token_account(MOCK_USDC_TOKEN_ACCOUNT.clone())
            .unwrap();
        controller
            .register_token_account(MOCK_USDT_TOKEN_ACCOUNT.clone())
            .unwrap();

        // Empty cache
        assert!(controller.evaluate_for_arbitrage().is_empty());

        // LP data is in cache, but no Token account balances
        let buy_lp = create_raydium_amm_lp();
        controller
            .update_liquidity_pool(LiquidityPool::RaydiumAmm(buy_lp.clone()))
            .unwrap();
        {
            let mut buy_lp_state = controller
                .arb_cache
                .get_lp_state(&buy_lp.address)
                .unwrap()
                .borrow_mut();
            buy_lp_state.healthy = true;
            buy_lp_state.base_token_digits = 11_000_000_000;
            buy_lp_state.quote_token_digits = 90_000_000;
            buy_lp_state.fee_ratio = 0.001;
            buy_lp_state.latest_upd_txn_sig = Some(Signature::new_unique());
        }

        let sell_lp = create_raydium_amm_lp();
        controller
            .update_liquidity_pool(LiquidityPool::RaydiumAmm(sell_lp.clone()))
            .unwrap();
        {
            let mut sell_lp_state = controller
                .arb_cache
                .get_lp_state(&sell_lp.address)
                .unwrap()
                .borrow_mut();
            sell_lp_state.healthy = true;
            sell_lp_state.base_token_digits = 9_000_000_000;
            sell_lp_state.quote_token_digits = 110_000_000;
            sell_lp_state.fee_ratio = 0.001;
            sell_lp_state.latest_upd_txn_sig = Some(Signature::new_unique());
        }
        assert!(controller.evaluate_for_arbitrage().is_empty());

        // Token accounts have balances, should return an arb.
        controller
            .arb_cache
            .update_token_account_balance(*buy_lp.base_token.mint(), 10_000_000_000);
        controller
            .arb_cache
            .update_token_account_balance(*sell_lp.quote_token.mint(), 100_000_000);

        assert_eq!(controller.evaluate_for_arbitrage().len(), 1);

        // Quote amount diff is too small, no arb.
        {
            let mut sell_lp_state = controller
                .arb_cache
                .get_lp_state(&sell_lp.address)
                .unwrap()
                .borrow_mut();
            sell_lp_state.base_token_digits = 10_900_000_000;
            sell_lp_state.quote_token_digits = 90_900_000;
        }
        assert!(controller.evaluate_for_arbitrage().is_empty());
    }

    #[test]
    fn test_min_arbitrage() {
        let controller = ArbitrageController {
            accounts: HashMap::new(),
            last_account_update_cache: LastUpdateSlotCache::new(),
            arb_cache: ArbitrageCache::new(),
            arb_executor: MockArbitrageExecutor { was_executed: false },
            metrics_collector: NoopMetricsCollector {},
            total_arb_iterations: MOCK_TOTAL_ARB_ITERATIONS,
            min_base_swap_amount: MOCK_MIN_BASE_SWAP_AMOUNT,
            min_quote_margin: MOCK_MIN_QUOTE_MARGIN,
        };

        let buy_lp = create_orca_whirlpool_lp();
        let mut buy_lp_state = LiquidityPoolState::new(LiquidityPool::OrcaWhirlpool(buy_lp));
        buy_lp_state.healthy = true;
        buy_lp_state.base_token_digits = 20_000_000_000;
        buy_lp_state.quote_token_digits = 100_000_000;
        buy_lp_state.fee_ratio = 0.01;

        let sell_lp = create_raydium_amm_lp();
        let mut sell_lp_state = LiquidityPoolState::new(LiquidityPool::RaydiumAmm(sell_lp));
        sell_lp_state.healthy = true;
        sell_lp_state.base_token_digits = 10_000_000_000;
        sell_lp_state.quote_token_digits = 200_000_000;
        sell_lp_state.fee_ratio = 0.02;

        let arb = controller
            .calculate_arbitrage(
                &buy_lp_state,
                &sell_lp_state,
                MOCK_BASE_TOKEN_BALANCE,
                MOCK_QUOTE_TOKEN_BALANCE,
            )
            .unwrap()
            .swap_quote_amount_margin();
        assert!((arb - controller.min_quote_margin).abs() < ONE_BP_DECIMAL);
    }

    #[test]
    fn test_arbitrage_edge_conditions() {
        let buy_lp = create_raydium_amm_lp();
        let mut buy_lp_state = LiquidityPoolState::new(LiquidityPool::RaydiumAmm(buy_lp));
        buy_lp_state.healthy = true;
        buy_lp_state.base_token_digits = 11_000_000_000;
        buy_lp_state.quote_token_digits = 90_000_000;
        buy_lp_state.fee_ratio = 0.001;

        let sell_lp = create_raydium_amm_lp();
        let mut sell_lp_state = LiquidityPoolState::new(LiquidityPool::RaydiumAmm(sell_lp));
        sell_lp_state.healthy = true;
        sell_lp_state.base_token_digits = 9_000_000_000;
        sell_lp_state.quote_token_digits = 110_000_000;
        sell_lp_state.fee_ratio = 0.001;

        let mut controller = ArbitrageController {
            accounts: HashMap::new(),
            last_account_update_cache: LastUpdateSlotCache::new(),
            arb_cache: ArbitrageCache::new(),
            arb_executor: MockArbitrageExecutor { was_executed: false },
            metrics_collector: NoopMetricsCollector {},
            total_arb_iterations: MOCK_TOTAL_ARB_ITERATIONS,
            min_base_swap_amount: MOCK_MIN_BASE_SWAP_AMOUNT,
            min_quote_margin: MOCK_MIN_QUOTE_MARGIN,
        };

        // Arbitrage
        assert!(controller
            .calculate_arbitrage(
                &buy_lp_state,
                &sell_lp_state,
                MOCK_BASE_TOKEN_BALANCE,
                MOCK_QUOTE_TOKEN_BALANCE
            )
            .is_some());

        // Low base token account balance
        assert!(controller
            .calculate_arbitrage(&buy_lp_state, &sell_lp_state, 0.001, MOCK_QUOTE_TOKEN_BALANCE)
            .is_none());

        // Low quote token account balance
        assert!(controller
            .calculate_arbitrage(&buy_lp_state, &sell_lp_state, MOCK_BASE_TOKEN_BALANCE, 0.001)
            .is_none());

        // Equal buy and sell pricess
        assert!(controller
            .calculate_arbitrage(
                &buy_lp_state,
                &buy_lp_state,
                MOCK_BASE_TOKEN_BALANCE,
                MOCK_QUOTE_TOKEN_BALANCE
            )
            .is_none());

        // quote margin is too small or even negative
        buy_lp_state.base_token_digits = sell_lp_state.base_token_digits;
        buy_lp_state.quote_token_digits = sell_lp_state.quote_token_digits;
        assert!(controller
            .calculate_arbitrage(
                &buy_lp_state,
                &sell_lp_state,
                MOCK_BASE_TOKEN_BALANCE,
                MOCK_QUOTE_TOKEN_BALANCE
            )
            .is_none());

        // Undefined buy fee ratio
        buy_lp_state.base_token_digits = sell_lp_state.base_token_digits * 2;
        buy_lp_state.quote_token_digits = sell_lp_state.quote_token_digits / 2;
        assert!(controller
            .calculate_arbitrage(
                &buy_lp_state,
                &sell_lp_state,
                MOCK_BASE_TOKEN_BALANCE,
                MOCK_QUOTE_TOKEN_BALANCE
            )
            .is_some());
        buy_lp_state.fee_ratio = f64::NAN;
        assert!(controller
            .calculate_arbitrage(
                &buy_lp_state,
                &sell_lp_state,
                MOCK_BASE_TOKEN_BALANCE,
                MOCK_QUOTE_TOKEN_BALANCE
            )
            .is_none());

        // Min quote margin is too high
        buy_lp_state.fee_ratio = 0.001;
        assert!(controller
            .calculate_arbitrage(
                &buy_lp_state,
                &sell_lp_state,
                MOCK_BASE_TOKEN_BALANCE,
                MOCK_QUOTE_TOKEN_BALANCE
            )
            .is_some());
        controller.min_quote_margin = 10000.0;
        assert!(controller
            .calculate_arbitrage(
                &buy_lp_state,
                &sell_lp_state,
                MOCK_BASE_TOKEN_BALANCE,
                MOCK_QUOTE_TOKEN_BALANCE
            )
            .is_none());
    }

    #[ignore]
    #[test]
    fn test_arb_with_specific_numbers1() {
        let mut buy_lp = create_raydium_amm_lp();
        buy_lp.min_base_token_balance = 1.0;
        buy_lp.min_quote_token_balance = 150.0;
        let mut buy_lp_state = LiquidityPoolState::new(LiquidityPool::RaydiumAmm(buy_lp));
        buy_lp_state.healthy = true;
        buy_lp_state.base_token_digits = 2830249902;
        buy_lp_state.quote_token_digits = 411651164;
        buy_lp_state.fee_ratio = 0.0025;

        let mut sell_lp = create_raydium_amm_lp();
        sell_lp.min_base_token_balance = 1.0;
        sell_lp.min_quote_token_balance = 150.0;
        let mut sell_lp_state = LiquidityPoolState::new(LiquidityPool::RaydiumAmm(sell_lp));
        sell_lp_state.healthy = true;
        sell_lp_state.base_token_digits = 3114303177264;
        sell_lp_state.quote_token_digits = 455560751457;
        sell_lp_state.fee_ratio = 0.0025;

        let controller = ArbitrageController {
            accounts: HashMap::new(),
            last_account_update_cache: LastUpdateSlotCache::new(),
            arb_cache: ArbitrageCache::new(),
            arb_executor: MockArbitrageExecutor { was_executed: false },
            metrics_collector: NoopMetricsCollector {},
            total_arb_iterations: MOCK_TOTAL_ARB_ITERATIONS,
            min_base_swap_amount: MOCK_MIN_BASE_SWAP_AMOUNT,
            min_quote_margin: MOCK_MIN_QUOTE_MARGIN,
        };

        let arb = controller
            .calculate_arbitrage(&buy_lp_state, &sell_lp_state, 10.0, 1000.0)
            .unwrap();
        assert!(arb.swap_base_qty - 1.8302499019999998 < ONE_BP_DECIMAL);
        assert!(arb.swap_buy_quote_amount - 266.9148295926596 < ONE_BP_DECIMAL);
        assert!(arb.swap_sell_quote_amount - 267.0148295926596 < ONE_BP_DECIMAL);
    }

    #[test]
    fn test_arb_with_specific_numbers2() {
        let mut buy_lp = create_raydium_amm_lp();
        buy_lp.min_base_token_balance = 1.0;
        buy_lp.min_quote_token_balance = 150.0;
        let mut buy_lp_state = LiquidityPoolState::new(LiquidityPool::RaydiumAmm(buy_lp));
        buy_lp_state.healthy = true;
        buy_lp_state.base_token_digits = 3127950881696;
        buy_lp_state.quote_token_digits = 453579516791;
        buy_lp_state.fee_ratio = 0.0025;

        let mut sell_lp = create_raydium_amm_lp();
        sell_lp.min_base_token_balance = 1.0;
        sell_lp.min_quote_token_balance = 150.0;
        let mut sell_lp_state = LiquidityPoolState::new(LiquidityPool::RaydiumAmm(sell_lp));
        sell_lp_state.healthy = true;
        sell_lp_state.base_token_digits = 20090320585335;
        sell_lp_state.quote_token_digits = 2941463207726;
        sell_lp_state.fee_ratio = 0.0025;

        let controller = ArbitrageController {
            accounts: HashMap::new(),
            last_account_update_cache: LastUpdateSlotCache::new(),
            arb_cache: ArbitrageCache::new(),
            arb_executor: MockArbitrageExecutor { was_executed: false },
            metrics_collector: NoopMetricsCollector {},
            total_arb_iterations: MOCK_TOTAL_ARB_ITERATIONS,
            min_base_swap_amount: MOCK_MIN_BASE_SWAP_AMOUNT,
            min_quote_margin: MOCK_MIN_QUOTE_MARGIN,
        };

        let arb = controller
            .calculate_arbitrage(&buy_lp_state, &sell_lp_state, 10.0, 1000.0)
            .unwrap();
        assert!(arb.swap_base_qty - 6.88925053962293 < ONE_BP_DECIMAL);
        assert!(arb.swap_buy_quote_amount - 1003.77225031499 < ONE_BP_DECIMAL);
        assert!(arb.swap_sell_quote_amount - 1003.87225031499 < ONE_BP_DECIMAL);
    }
}
