use {
    super::ArbitrageExecutor,
    common::{
        message::ArbitrageMessage,
        serializer::{BinarySerializer, Serializer, BUFFER_SIZE},
    },
    log::info,
    memmap2::{MmapMut, MmapOptions},
    serde::Deserialize,
    serde_json::Value,
    std::fs::OpenOptions,
};

#[derive(Deserialize)]
struct ArbitragePublisherConfig {
    file_name: String,
}

impl ArbitragePublisherConfig {
    pub fn new(json: Value) -> Self {
        serde_json::from_value(json).unwrap()
    }
}

pub struct ArbitragePublisher {
    serializer: BinarySerializer,
    buffer: Vec<u8>,
    publisher: MmapMut,
}

impl ArbitrageExecutor for ArbitragePublisher {
    fn new(config: Value) -> Self {
        info!("Creating ArbitragePublisher. Config: `{}`", config);
        let config = ArbitragePublisherConfig::new(config);

        let dest_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(config.file_name)
            .unwrap();
        dest_file.set_len(BUFFER_SIZE as u64).unwrap();

        ArbitragePublisher {
            serializer: BinarySerializer::new(),
            buffer: vec![0; BUFFER_SIZE],
            publisher: unsafe { MmapOptions::new().len(BUFFER_SIZE).map_mut(&dest_file) }.unwrap(),
        }
    }

    fn execute(&mut self, arb_msg: &ArbitrageMessage) -> anyhow::Result<()> {
        self.serializer.serialize_arbitrage(arb_msg, &mut self.buffer)?;
        self.publisher.copy_from_slice(&self.buffer[..]);
        info!("Published ArbitrageMessage ({:?})", arb_msg);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use {
        super::ArbitragePublisher,
        crate::arbitrage::{ArbitrageExecutor, ArbitrageMessage},
        common::{
            message::SequenceId, test_util::tests::create_liquidity_pool, AccountType, AmountType,
            LiquidityPoolState,
        },
        serde_json::Value,
        tempfile::NamedTempFile,
    };

    #[test]
    fn test_execute() {
        let data_file = NamedTempFile::new().unwrap();
        let config_str = format!(
            r#"
            {{
                "file_name": "{}"
            }}"#,
            data_file.path().to_str().unwrap()
        );
        let config: Value = serde_json::from_str(&config_str).unwrap();
        let mut publisher = ArbitragePublisher::new(config);

        let mut seqnum = SequenceId::new();
        for i in 1..1000 {
            let arbitrage = ArbitrageMessage::new(
                seqnum.increment_and_get(),
                i,
                &LiquidityPoolState::new(create_liquidity_pool(AccountType::OrcaWhirlpoolAccount)),
                &LiquidityPoolState::new(create_liquidity_pool(AccountType::RaydiumAmmPoolAccount)),
                i as f64 + 67.89 as AmountType,
                i as f64 + 1.01 as AmountType,
                i as f64 + 2.01 as AmountType,
            );
            publisher.execute(&arbitrage).unwrap();
            publisher.execute(&arbitrage).unwrap();
        }
    }
}
