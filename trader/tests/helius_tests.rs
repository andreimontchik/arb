#[cfg(test)]
mod tests {
    use {
        common::{
            config::{to_raydium_amm, RaydiumAmmConfig, TokenAccountConfig, TokensConfig, WalletConfig},
            message::{ArbitrageMessage, SequenceId},
            test_util::tests::create_mock_tokens_hashmap,
            AmountType, LiquidityPool, LiquidityPoolState,
        },
        serde_json::Value,
        solana_sdk::pubkey::Pubkey,
        std::{str::FromStr, time::Instant},
        trader::{
            gateway::{Gateway, HeliusGateway},
            processor,
            test_util::tests::{
                MOCK_ARB_CU_LIMIT, MOCK_MAX_CU_PRICE, MOCK_MIN_CU_PRICE, MOCK_SWAP_CU_LIMIT,
            },
            Context, ContextConfig,
        },
    };

    fn get_config(cluster: &str) -> Value {
        let config = format!(
            r#"{{
            "api_key": "640bbf81-f665-4d01-84a8-180f1438a255",
            "cluster": "{}",
            "txn_skip_preflight": true,
            "txn_max_retries": 1
        }}"#,
            cluster
        );

        serde_json::from_str(&config).unwrap()
    }

    // Latest block hash
    fn test_get_latest_blockhash(cluster: &str) {
        let gateway = HeliusGateway::new(get_config(cluster));
        let start = Instant::now();
        let block_hash = gateway.get_latest_blockhash().unwrap();
        let duration = start.elapsed();
        println!(
            "Latest blockhash for {}: ({}). Duration: {} ms.",
            cluster,
            block_hash,
            duration.as_millis()
        );
    }

    #[test]
    #[ignore]
    fn test_get_latest_blockhash_devnet() {
        test_get_latest_blockhash("devnet");
    }

    #[test]
    #[ignore]
    fn test_get_latest_blockhash_mainnet() {
        test_get_latest_blockhash("mainnet");
    }

    // Recent priority fee
    fn test_get_recent_priority_fee(account: &Pubkey, cluster: &str) {
        let gateway = HeliusGateway::new(get_config(cluster));
        let start = Instant::now();
        let fee = gateway.get_recent_cu_price(account).unwrap();
        let duration = start.elapsed();
        println!(
            "Recent priority fee for account {} on {} is ({:?}). Duration: {} ms.",
            account,
            cluster,
            fee,
            duration.as_millis()
        );
    }

    #[test]
    #[ignore]
    fn test_get_recent_priority_fee_for_spl_token_program_on_mainnet() {
        test_get_recent_priority_fee(
            &Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
            "mainnet",
        );
    }

    #[test]
    #[ignore]
    fn test_simulate_arb_on_mainnet() {
        let cluster = "mainnet";
        let gateway = HeliusGateway::new(get_config(cluster));

        let context_config = ContextConfig {
            spl_token_program_id: Pubkey::new_unique().to_string(),
            arb_cu_limit: MOCK_ARB_CU_LIMIT,
            min_cu_price: MOCK_MIN_CU_PRICE,
            max_cu_price: MOCK_MAX_CU_PRICE,
            swap_cu_limit: MOCK_SWAP_CU_LIMIT,
            update_period_sec: 1,
        };

        let tokens_config =
            TokensConfig::load("/home/andrei/work/src/research/config/mainnet/tokens.json");

        let raydium_config =
            RaydiumAmmConfig::load("/home/andrei/work/src/research/config/mainnet/raydium.json");
        let buy_lp = to_raydium_amm(
            &raydium_config.amm_program_id,
            &raydium_config.authority,
            raydium_config.min_base_token_balance,
            raydium_config.min_quote_token_balance,
            &raydium_config.amm_pools[0],
            &create_mock_tokens_hashmap(),
        )
        .unwrap();
        let sell_lp = to_raydium_amm(
            &raydium_config.amm_program_id,
            &raydium_config.authority,
            raydium_config.min_base_token_balance,
            raydium_config.min_quote_token_balance,
            &raydium_config.amm_pools[1],
            &create_mock_tokens_hashmap(),
        )
        .unwrap();

        let mut tokens: Vec<TokenAccountConfig> = vec![];
        let mut token = TokenAccountConfig {
            code: LiquidityPool::base_token(&buy_lp).code().to_string(),
            address: Pubkey::new_unique().to_string(),
            min_amount: 0.01,
        };
        tokens.push(token);
        token = TokenAccountConfig {
            code: LiquidityPool::quote_token(&buy_lp).code().to_string(),
            address: Pubkey::new_unique().to_string(),
            min_amount: 1.0,
        };
        tokens.push(token);
        token = TokenAccountConfig {
            code: LiquidityPool::quote_token(&sell_lp).code().to_string(),
            address: Pubkey::new_unique().to_string(),
            min_amount: 1.0,
        };
        tokens.push(token);

        let wallet_config = WalletConfig {
            keypair: "/home/andrei/work/src/research/config/localnet/wallet_keypair.json".to_string(),
            token_accounts: "/home/andrei/work/src/research/config/localnet/token_accounts.json"
                .to_string(),
        };

        let mut context =
            Context::new(&context_config, &wallet_config, &tokens_config, &raydium_config).unwrap();

        context.update_recent_cu_price(&LiquidityPool::program_id(&buy_lp), 100_000);
        context.update_recent_cu_price(&LiquidityPool::program_id(&sell_lp), 200_000);

        let block_hash = gateway.get_latest_blockhash().unwrap();
        context.update_latest_blockhash(block_hash);

        let msg = ArbitrageMessage::new(
            SequenceId::new(),
            1,
            &LiquidityPoolState::new(buy_lp.clone()),
            &LiquidityPoolState::new(sell_lp.clone()),
            1 as AmountType,
            10 as AmountType,
            11 as AmountType,
        );

        let (txn, _, _) = processor::create_arbitrage_transaction(&context, &msg).unwrap();

        let start = Instant::now();
        let result = gateway.simulate_transaction(&txn);
        let duration = start.elapsed();
        println!(
            "The Arbitrage transaction simulation result on {}: ({:?}). Duration: {} ms.",
            cluster,
            result,
            duration.as_millis()
        );
    }
}
