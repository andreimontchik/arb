use {
    common::{
        config::{to_token, RaydiumAmmConfig, TokensConfig, WalletConfig},
        metrics::statsd_metrics_collector::StatsdMetricsCollector,
        serializer::BinarySerializer,
        Token, TokenCode,
    },
    env_logger,
    log::{error, info},
    signal_hook::{consts::signal::SIGTERM, flag},
    std::{
        collections::HashMap,
        env,
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc, Mutex,
        },
        thread,
        time::Duration,
    },
    trader::{
        gateway::HeliusGateway, run_main_loop, Context, ContextUpdater, MmapReceiver, Processor,
        Receiver, TraderConfig,
    },
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Missing the config file startup argument!");
    }

    // Initialize logging
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp_micros()
        .init();

    let config_file = &args[1];
    let config = TraderConfig::load(config_file);

    let wallet_config = WalletConfig::load(&config.wallet);

    let mut tokens: HashMap<TokenCode, Token> = HashMap::new();
    let tokens_config = TokensConfig::load(&config.tokens);
    for token_config in tokens_config.tokens() {
        let token = to_token(&token_config).unwrap();
        tokens.insert(token.code(), token);
    }

    let raydium_config = RaydiumAmmConfig::load(&config.raydium);

    // Create the Context
    let context = Context::new(&config.context, &wallet_config, &tokens_config, &raydium_config)
        .unwrap_or_else(|err| panic!("Failed to create the Context! {}", err));
    let context: Arc<Mutex<Context>> = Arc::new(Mutex::new(context));

    let mut context_updater = ContextUpdater::new(config.context.update_period_sec, context.clone());
    context_updater
        .start::<HeliusGateway, StatsdMetricsCollector>(config.gateway.clone(), config.metrics.clone());

    // Register shutdown hooks
    let shutdown = Arc::new(AtomicBool::new(false));
    flag::register(SIGTERM, shutdown.clone()).expect("Error setting the SIGTERM handler!");
    let ctrlc_flag = shutdown.clone();
    ctrlc::set_handler(move || {
        ctrlc_flag.store(true, Ordering::Relaxed);
    })
    .expect("Error setting Ctrl-C handler!");

    // Create receiver
    let mut receiver = MmapReceiver::<BinarySerializer>::try_new(&config.receiver);
    while !shutdown.load(Ordering::Relaxed) && receiver.is_none() {
        receiver = MmapReceiver::<BinarySerializer>::try_new(&config.receiver);
        error!(
            "Failed to create the Receiver. Will try again in {} seconds",
            config.receiver_create_interval_sec
        );
        thread::sleep(Duration::from_secs(config.receiver_create_interval_sec));
    }

    let mut receiver = receiver.unwrap_or_else(|| {
        info!("The Receiver was not created. Terminating.");
        std::process::exit(1);
    });

    let processor = Processor::new(config.gateway, config.metrics, config.simulation, context.clone());

    info!("Staring the main loop");
    let (total_received, total_processed, total_failed) =
        run_main_loop::<BinarySerializer, HeliusGateway, StatsdMetricsCollector>(
            shutdown,
            &mut receiver,
            config.receiver_sleep_interval_us,
            processor,
        );

    context_updater.stop();

    info!(
        "Shutting down. Total (recevied, processed, failed : ({}, {}, {}).",
        total_received, total_processed, total_failed
    );
}
