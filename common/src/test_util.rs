pub mod tests {

    use {
        crate::{
            config::RaydiumAmmPoolConfig,
            message::{ArbitrageMessage, Message},
            serializer::{self, Serializer},
            AccountType, LiquidityGroupCode, LiquidityPool, LiquidityPoolState, OrcaWhirlpoolLp,
            RaydiumAmmLp, Token, TokenAccount, TokenCode, Wallet,
        },
        anyhow::Result,
        ported_from_raydium::{AmmState, AmmStatus},
        raydium_amm_interface::{AmmInfo, Fees, OutPutData},
        solana_sdk::{
            program_option::COption,
            pubkey::Pubkey,
            signature::{Keypair, Signature},
        },
        spl_token::state::{Account, AccountState},
        std::{
            collections::HashMap,
            time::{SystemTime, UNIX_EPOCH},
        },
        whirlpool_interface::{Whirlpool, WhirlpoolAccount, WhirlpoolRewardInfo},
    };

    // Mock Tokens
    #[allow(non_upper_case_globals)]
    pub static MOCK_wSOL_TOKEN: Token = Token {
        code: TokenCode::wSOL,
        mint: Pubkey::new_from_array([1; 32]),
        decimals: 9,
    };

    #[allow(non_upper_case_globals)]
    pub const MOCK_stSOL_TOKEN: Token = Token {
        code: TokenCode::stSOL,
        mint: Pubkey::new_from_array([2; 32]),
        decimals: 9,
    };

    pub const MOCK_USDC_TOKEN: Token = Token {
        code: TokenCode::USDC,
        mint: Pubkey::new_from_array([3; 32]),
        decimals: 6,
    };
    pub const MOCK_USDT_TOKEN: Token = Token {
        code: TokenCode::USDT,
        mint: Pubkey::new_from_array([4; 32]),
        decimals: 6,
    };

    #[allow(non_upper_case_globals)]
    pub static MOCK_wSOL_TOKEN_ACCOUNT: TokenAccount = TokenAccount {
        code: TokenCode::wSOL,
        address: Pubkey::new_from_array([5; 32]),
        min_balance: 0.01,
    };

    pub static MOCK_USDC_TOKEN_ACCOUNT: TokenAccount = TokenAccount {
        code: TokenCode::USDC,
        address: Pubkey::new_from_array([6; 32]),
        min_balance: 1.0,
    };

    pub static MOCK_USDT_TOKEN_ACCOUNT: TokenAccount = TokenAccount {
        code: TokenCode::USDT,
        address: Pubkey::new_from_array([7; 32]),
        min_balance: 1.0,
    };

    pub fn create_mock_tokens_hashmap() -> HashMap<TokenCode, Token> {
        let mut result: HashMap<TokenCode, Token> = HashMap::new();
        result.insert(MOCK_wSOL_TOKEN.code, MOCK_wSOL_TOKEN.clone());
        result.insert(MOCK_USDC_TOKEN.code, MOCK_USDC_TOKEN);
        result.insert(MOCK_USDT_TOKEN.code, MOCK_USDT_TOKEN);
        result
    }

    pub fn create_mock_wallet() -> Wallet {
        let keypair = Keypair::new();

        let mut tokens: HashMap<TokenCode, TokenAccount> = HashMap::new();
        tokens.insert(
            TokenCode::wSOL,
            TokenAccount {
                code: TokenCode::wSOL,
                address: Pubkey::new_unique(),
                min_balance: 0.01,
            },
        );
        tokens.insert(
            TokenCode::USDC,
            TokenAccount {
                code: TokenCode::USDC,
                address: Pubkey::new_unique(),
                min_balance: 1.0,
            },
        );
        tokens.insert(
            TokenCode::USDT,
            TokenAccount {
                code: TokenCode::USDT,
                address: Pubkey::new_unique(),
                min_balance: 1.0,
            },
        );

        Wallet {
            keypair,
            token_accounts: tokens,
        }
    }
    pub struct MockSerializer {}

    impl Serializer for MockSerializer {
        fn new() -> Self {
            MockSerializer {}
        }

        fn serialize_message(&mut self, _msg: &Message, _buffer: &mut Vec<u8>) -> Result<usize> {
            Ok(0)
        }

        fn serialize_block_update(
            &mut self,
            _msg: &crate::message::BlockUpdateMessage,
            _buffer: &mut Vec<u8>,
        ) -> Result<usize> {
            Ok(0)
        }

        fn serialize_account_update(
            &mut self,
            _msg: &crate::message::AccountUpdateMessage,
            _buffer: &mut Vec<u8>,
        ) -> Result<usize> {
            Ok(0)
        }

        fn serialize_liquidity_pool(
            &mut self,
            _lp: &LiquidityPool,
            _buffer: &mut Vec<u8>,
        ) -> Result<usize> {
            Ok(0)
        }

        fn serialize_arbitrage(
            &mut self,
            _arbitrage: &ArbitrageMessage,
            _buffer: &mut Vec<u8>,
        ) -> Result<usize> {
            Ok(0)
        }

        fn deserialize_message_type(&self, _buffer: &[u8]) -> Result<serializer::MessageType> {
            unimplemented!()
        }
    }

    pub fn create_raydium_amm_lp() -> RaydiumAmmLp {
        RaydiumAmmLp {
            liquidity_group: LiquidityGroupCode::SOL_USD,
            program_id: Pubkey::new_unique(),
            name: "Test RaydiumAmmPool account name".to_string(),
            address: Pubkey::new_unique(),
            authority: Pubkey::new_unique(),
            base_token: MOCK_wSOL_TOKEN.clone(),
            base_token_vault: Pubkey::new_unique(),
            min_base_token_balance: 0.01,
            quote_token: MOCK_USDC_TOKEN,
            quote_token_vault: Pubkey::new_unique(),
            min_quote_token_balance: 1.0,
        }
    }

    pub fn create_orca_whirlpool_lp() -> OrcaWhirlpoolLp {
        OrcaWhirlpoolLp {
            liquidity_group: LiquidityGroupCode::SOL_USD,
            name: "Test OrcaWhirlpool account name".to_string(),
            address: Pubkey::new_unique(),
            base_token: MOCK_wSOL_TOKEN.clone(),
            min_base_token_balance: 0.001,
            quote_token: MOCK_USDC_TOKEN,
            min_quote_token_balance: 1.0,
            tick: 2,
        }
    }

    pub fn create_liquidity_pool(account_type: AccountType) -> LiquidityPool {
        match account_type {
            AccountType::OrcaWhirlpoolAccount => {
                LiquidityPool::OrcaWhirlpool(create_orca_whirlpool_lp())
            }
            AccountType::RaydiumAmmPoolAccount => LiquidityPool::RaydiumAmm(create_raydium_amm_lp()),
            _ => panic!("Unexpected account type {:?}!", account_type),
        }
    }

    pub fn create_lp_state(lp: LiquidityPool) -> LiquidityPoolState {
        let base_token_digits = 10 * 10u64.pow(LiquidityPool::base_token(&lp).decimals() as u32);
        let quote_token_digits = 100 * 10u64.pow(LiquidityPool::quote_token(&lp).decimals() as u32);
        let mut result = LiquidityPoolState::new(lp);
        result.healthy = true;
        result.base_token_digits = base_token_digits;
        result.quote_token_digits = quote_token_digits;
        result.fee_ratio = 0.001;
        result.latest_upd_txn_sig = Some(Signature::new_unique());
        result
    }

    pub fn create_orca_whirlpool_account() -> WhirlpoolAccount {
        let now = SystemTime::now();
        let duration_since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let seconds_since_epoch = duration_since_epoch.as_secs();

        WhirlpoolAccount(Whirlpool {
            whirlpools_config: Pubkey::new_unique(),
            whirlpool_bump: [255],
            tick_spacing: 4,
            tick_spacing_seed: [0, 4], // tick_spacing  as byte array
            fee_rate: 10000,           // 1%
            protocol_fee_rate: 200,    // 2% of fee_rate
            liquidity: 1000,
            sqrt_price: 123456789,
            tick_current_index: -8,
            protocol_fee_owed_a: 1234,
            protocol_fee_owed_b: 5678,
            token_mint_a: Pubkey::new_unique(),
            token_vault_a: Pubkey::new_unique(),
            fee_growth_global_a: 9123,
            token_mint_b: Pubkey::new_unique(),
            token_vault_b: Pubkey::new_unique(),
            fee_growth_global_b: 4567,
            reward_last_updated_timestamp: seconds_since_epoch,
            reward_infos: [
                WhirlpoolRewardInfo {
                    mint: Pubkey::new_unique(),
                    vault: Pubkey::new_unique(),
                    authority: Pubkey::new_unique(),
                    emissions_per_second_x64: 1,
                    growth_global_x64: 2,
                },
                WhirlpoolRewardInfo {
                    mint: Pubkey::new_unique(),
                    vault: Pubkey::new_unique(),
                    authority: Pubkey::new_unique(),
                    emissions_per_second_x64: 1,
                    growth_global_x64: 2,
                },
                WhirlpoolRewardInfo {
                    mint: Pubkey::new_unique(),
                    vault: Pubkey::new_unique(),
                    authority: Pubkey::new_unique(),
                    emissions_per_second_x64: 1,
                    growth_global_x64: 2,
                },
            ],
        })
    }

    pub fn create_raydium_amm_info() -> AmmInfo {
        AmmInfo {
            status: AmmStatus::Initialized.into_u64(),
            nonce: 2,
            order_num: 3,
            depth: 4,
            coin_decimals: 5, // for base token
            pc_decimals: 6,   // for quote token
            state: AmmState::IdleState.into_u64(),
            reset_flag: 8,
            min_size: 9,
            vol_max_cut_ratio: 10,
            amount_wave: 11,
            coin_lot_size: 12,
            pc_lot_size: 13,
            min_price_multiplier: 14,
            max_price_multiplier: 15,
            sys_decimal_value: 16,
            fees: Fees {
                min_separate_numerator: 17,
                min_separate_denominator: 18,
                trade_fee_numerator: 19,
                trade_fee_denominator: 20,
                pnl_numerator: 21,
                pnl_denominator: 22,
                swap_fee_numerator: 23,
                swap_fee_denominator: 24,
            },
            out_put: OutPutData {
                need_take_pnl_coin: 25,
                need_take_pnl_pc: 26,
                total_pnl_pc: 27,
                total_pnl_coin: 28,
                pool_open_time: 29,
                punish_pc_amount: 30,
                punish_coin_amount: 31,
                orderbook_to_init_time: 32,
                swap_coin_in_amount: 33,
                swap_pc_out_amount: 34,
                swap_take_pc_fee: 35,
                swap_pc_in_amount: 36,
                swap_coin_out_amount: 37,
                swap_take_coin_fee: 38,
            },
            token_coin: Pubkey::new_unique(),
            token_pc: Pubkey::new_unique(),
            coin_mint: Pubkey::new_unique(),
            pc_mint: Pubkey::new_unique(),
            lp_mint: Pubkey::new_unique(),
            open_orders: Pubkey::new_unique(),
            market: Pubkey::new_unique(),
            serum_dex: Pubkey::new_unique(),
            target_orders: Pubkey::new_unique(),
            withdraw_queue: Pubkey::new_unique(),
            token_temp_lp: Pubkey::new_unique(),
            amm_owner: Pubkey::new_unique(),
            lp_amount: 39,
            client_order_id: 40,
            padding: [0, 0],
        }
    }

    pub fn create_raydium_amm_pool_config(
        lg_code: LiquidityGroupCode,
        base_token: TokenCode,
        quote_token: TokenCode,
    ) -> RaydiumAmmPoolConfig {
        RaydiumAmmPoolConfig {
            enabled: true,
            liquidity_group: lg_code.to_string(),
            address: Pubkey::new_unique().to_string(),
            base_token: base_token.to_string(),
            base_token_vault: Pubkey::new_unique().to_string(),
            quote_token: quote_token.to_string(),
            quote_token_vault: Pubkey::new_unique().to_string(),
        }
    }

    pub fn create_spl_account(amount: u64) -> Account {
        Account {
            mint: Pubkey::new_unique(),
            owner: Pubkey::new_unique(),
            amount,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        }
    }
}
