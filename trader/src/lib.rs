mod context;

pub mod gateway;
pub mod processor;
pub mod receiver;
pub mod test_util;

use {
    common::{
        message::{Message, SequenceId},
        metrics::{MetricKey, MetricsCollector},
        read_from_file,
        serializer::Serializer,
    },
    gateway::Gateway,
    log::{debug, error, info, warn},
    serde_derive::Deserialize,
    serde_json::Value,
    std::{
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
        thread,
        time::Duration,
    },
};
pub use {
    context::{Context, ContextUpdater},
    processor::Processor,
    receiver::{MmapReceiver, Receiver},
};

pub type CounterType = u64;
pub type TimeIntervalType = u64;
pub type ComputedUnitsLimitType = u32;
pub type ComputedUnitsPriceType = u64;

enum TraderMetricKey {
    SimulateTransactionDuration,
    SubmitTransactionDuration,
    GetLatestBlockHashDuration,
    GetRecentCuPriceDuration,
}

impl MetricKey for TraderMetricKey {
    fn to_str(&self) -> &str {
        match self {
            TraderMetricKey::SimulateTransactionDuration => "simulate-transaction-us",
            TraderMetricKey::SubmitTransactionDuration => "submit-transaction-us",
            TraderMetricKey::GetLatestBlockHashDuration => "get-latest-blockhash-us",
            TraderMetricKey::GetRecentCuPriceDuration => "get-recent-cu-price-us",
        }
    }
}

#[derive(Deserialize)]
pub struct TraderConfig {
    pub context: ContextConfig,
    pub receiver: Value,
    pub receiver_create_interval_sec: TimeIntervalType,
    pub receiver_sleep_interval_us: TimeIntervalType,
    pub tokens: String,
    pub wallet: String,
    pub raydium: String,
    pub gateway: Value,
    pub metrics: Value,
    pub simulation: bool,
}

impl TraderConfig {
    pub fn load(config_file_name: &str) -> Self {
        info!("Lading the TraderConfig from ({config_file_name}).");
        let config_file_contents = read_from_file(config_file_name);
        serde_json::from_str(&config_file_contents).unwrap()
    }
}

#[derive(Deserialize)]
pub struct ContextConfig {
    pub spl_token_program_id: String,
    pub arb_cu_limit: ComputedUnitsLimitType,
    pub swap_cu_limit: ComputedUnitsLimitType,
    pub min_cu_price: ComputedUnitsPriceType,
    pub max_cu_price: ComputedUnitsPriceType,
    pub update_period_sec: TimeIntervalType,
}

impl ContextConfig {
    pub fn new(config: Value) -> Self {
        serde_json::from_value(config).unwrap()
    }
}

pub fn run_main_loop<T: Serializer, U: Gateway, M: MetricsCollector>(
    shutdown: Arc<AtomicBool>,
    receiver: &mut MmapReceiver<T>,
    receiver_sleep_interval_us: TimeIntervalType,
    processor: Processor<U, M>,
) -> (CounterType, CounterType, CounterType) {
    let mut received_counter: CounterType = 0;
    let mut processed_counter: CounterType = 0;
    let mut failed_counter: CounterType = 0;
    let mut last_msg_seq_opt: Option<SequenceId> = None;
    while !shutdown.load(Ordering::Relaxed) {
        match receiver.receive() {
            Ok(msg) => match msg {
                Some(msg) => match &last_msg_seq_opt {
                    Some(last_msg_seq) => {
                        let msg_seq = Message::sequence_id(&msg);
                        if msg_seq > last_msg_seq {
                            info!("Processing the message ({})", msg);

                            received_counter += 1;

                            if !SequenceId::is_msg_gap(last_msg_seq, msg_seq) {
                                warn!("Message gap detected: ({}, {})", last_msg_seq, msg_seq);
                            }

                            if let Err(error) = processor.process(&msg) {
                                failed_counter += 1;
                                error!("Failed to process the message ({})! {:?}", &msg, error)
                            } else {
                                processed_counter += 1;
                            }
                            last_msg_seq_opt = Some(*msg_seq);
                        } else {
                            debug!("Discarding the duplicate message {}: ({})", received_counter, msg);
                            thread::sleep(Duration::from_micros(receiver_sleep_interval_us));
                        }
                    }
                    None => {
                        info!("Discarding the first message {}: ({})", received_counter, msg);
                        last_msg_seq_opt = Some(*Message::sequence_id(&msg));
                    }
                },
                None => {
                    debug!(
                        "Have not received any messages. Sleeping for {} us",
                        receiver_sleep_interval_us
                    );
                    thread::sleep(Duration::from_micros(receiver_sleep_interval_us));
                }
            },
            Err(err) => {
                error!("Failed to process received message! {:?}", err.to_string());
                //TODO: parametrize
                thread::sleep(Duration::from_secs(1));
            }
        }
    }

    (received_counter, processed_counter, failed_counter)
}

#[cfg(test)]
mod tests {
    use {
        crate::{ComputedUnitsLimitType, ComputedUnitsPriceType, TimeIntervalType, TraderConfig},
        std::fs,
        tempfile::NamedTempFile,
    };

    #[test]
    fn test_trader_configuration() {
        const SPL_TOKEN_PROGRAM_ID: &str = "SPL Token Program Id";
        const RECEIVER_CONFIG: &str = "Test Receiver Config";
        const RECEIVER_CREATE_INTERVAL: TimeIntervalType = 123456;
        const RECEIVER_SLEEP_INTERVAL: TimeIntervalType = 789012;
        const ARB_CU_LIMIT: ComputedUnitsLimitType = 345678;
        const MIN_CU_PRICE: ComputedUnitsPriceType = 500_000;
        const MAX_CU_PRICE: ComputedUnitsPriceType = 1_000_000;
        const SWAP_CU_LIMIT: ComputedUnitsLimitType = 50123;
        const UPDATE_PERIOD_SEC: TimeIntervalType = 112;
        const RAYDIUM_CONFIG_FILE: &str = "Raydium Config file";
        const WALLET_CONFIG_FILE: &str = "Wallet Config file";
        const GATEWAY_CONFIG: &str = "Gateway Config";
        const TOKENS_CONFIG_FILE: &str = "Tokens Config file";
        const METRICS_CONFIG: &str = "Metrics configuration";

        let config_str = format!(
            r#"
            {{
                "context": {{
                    "spl_token_program_id" : "{}",
                    "arb_cu_limit": {},
                    "swap_cu_limit": {},
                    "min_cu_price": {},
                    "max_cu_price": {},
                    "update_period_sec": {}
                }},
                "receiver": "{}",
                "receiver_create_interval_sec": {},
                "receiver_sleep_interval_us": {},
                "tokens": "{}",
                "wallet": "{}",
                "raydium": "{}",
                "gateway": "{}",
                "metrics": "{}",
                "simulation": true
            }}"#,
            SPL_TOKEN_PROGRAM_ID,
            ARB_CU_LIMIT,
            SWAP_CU_LIMIT,
            MIN_CU_PRICE,
            MAX_CU_PRICE,
            UPDATE_PERIOD_SEC,
            RECEIVER_CONFIG,
            RECEIVER_CREATE_INTERVAL,
            RECEIVER_SLEEP_INTERVAL,
            TOKENS_CONFIG_FILE,
            WALLET_CONFIG_FILE,
            RAYDIUM_CONFIG_FILE,
            GATEWAY_CONFIG,
            METRICS_CONFIG
        );

        let config_file = NamedTempFile::new().unwrap();
        fs::write(config_file.path(), config_str).unwrap();

        let config = TraderConfig::load(config_file.path().to_str().unwrap());
        assert_eq!(config.context.spl_token_program_id, SPL_TOKEN_PROGRAM_ID);
        assert_eq!(config.receiver, RECEIVER_CONFIG);
        assert_eq!(config.receiver_create_interval_sec, RECEIVER_CREATE_INTERVAL);
        assert_eq!(config.receiver_sleep_interval_us, RECEIVER_SLEEP_INTERVAL);
        assert_eq!(config.context.arb_cu_limit, ARB_CU_LIMIT);
        assert_eq!(config.context.max_cu_price, MAX_CU_PRICE);
        assert_eq!(config.tokens, TOKENS_CONFIG_FILE);
        assert_eq!(config.wallet, WALLET_CONFIG_FILE);
        assert_eq!(config.raydium, RAYDIUM_CONFIG_FILE);
        assert_eq!(config.gateway, GATEWAY_CONFIG);
        assert!(config.simulation);
    }
}
