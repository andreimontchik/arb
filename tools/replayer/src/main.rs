use {
    common::{
        config::{
            to_orca_whirlpool, to_raydium_amm, to_token, OrcaConfig, RaydiumAmmConfig,
            TokenAccountsConfig, TokensConfig,
        },
        message::Message,
        metrics::NoopMetricsCollector,
        serializer::CsvSerializer,
        Token, TokenAccount, TokenCode,
    },
    plugin::{
        arbitrage::ArbitragePersister,
        processor::{ArbitrageController, Processor},
    },
    replayer::{process, ReplayerConfig},
    std::{collections::HashMap, env, time::Instant},
};

fn main() {
    println!("Starting the Replayer.");

    solana_logger::setup_with_default("info");

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Invalid startup argument!! Usage: replayer <CONFIG_FILE> <MESSAGE_FILE>");
    }

    let config_file = &args[1];
    println!("Using the config file ({config_file}).");
    let config = ReplayerConfig::load(config_file);

    let message_file = &args[2];

    // let mut processor: ArbitrageController<NoopMetricsCollector, ArbitragePublisher> = ArbitrageController::new(&config.processor);
    let mut processor: ArbitrageController<NoopMetricsCollector, ArbitragePersister<CsvSerializer>> =
        ArbitrageController::new(&config.processor);

    let mut tokens: HashMap<TokenCode, Token> = HashMap::new();
    let tokens_config = TokensConfig::load(&config.tokens);
    for token_config in tokens_config.tokens() {
        let token = to_token(&token_config).unwrap();
        tokens.insert(token.code(), token);
    }

    let mut token_accounts: HashMap<TokenCode, TokenAccount> = HashMap::new();
    let token_accounts_config = TokenAccountsConfig::load(&config.token_accounts);
    for token_account_config in token_accounts_config.token_accounts() {
        let token_account = TokenAccount::try_from(token_account_config).unwrap();
        token_accounts.insert(token_account.code(), token_account.clone());
        processor
            .process(Message::TokenAccountConfiguration(token_account))
            .unwrap();
    }

    //    fund_token_accounts(&tokens, &token_accounts, config.token_funding, &mut processor);

    if !config.orca.is_empty() {
        let orca_config = OrcaConfig::load(&config.orca);

        for config in &orca_config.whirlpools {
            if config.enabled {
                println!("Registeding Orca Whirlpool ({:?})", config);

                let lp = to_orca_whirlpool(config, &tokens).unwrap();
                processor
                    .process(Message::LiquidityPoolConfiguration(lp))
                    .unwrap();
            } else {
                println!("Ignoring the disabled Orca Whirlpool ({:?})", config);
            }
        }
    };

    if !config.raydium.is_empty() {
        let raydium_config = RaydiumAmmConfig::load(&config.raydium);

        for amm_pool_config in raydium_config.amm_pools {
            if amm_pool_config.enabled {
                println!("Registeding Raydium AMM ({:?})", amm_pool_config);
                let lp = to_raydium_amm(
                    &raydium_config.amm_program_id,
                    &raydium_config.authority,
                    raydium_config.min_base_token_balance,
                    raydium_config.min_quote_token_balance,
                    &amm_pool_config,
                    &tokens,
                )
                .unwrap();
                processor
                    .process(Message::LiquidityPoolConfiguration(lp))
                    .unwrap();
            } else {
                println!("Ignoring the disabled Raydium AMM ({:?})", amm_pool_config);
            }
        }
    }

    let start = Instant::now();

    let (processed_counter, failed_counter) = process(processor, &message_file);

    println!(
        "The Replayer completed in {} sec. Processed/discarded msg count: {}/{}",
        start.elapsed().as_secs(),
        processed_counter,
        failed_counter,
    );
}
