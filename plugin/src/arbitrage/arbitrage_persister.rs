use {
    super::{ArbitrageError, ArbitrageExecutor, ArbitrageMessage, Result},
    chrono::Utc,
    common::{
        serializer::{Serializer, SerializerError, BUFFER_SIZE},
        EOL,
    },
    log::info,
    serde_derive::Deserialize,
    serde_json::{self, Value},
    std::{fs::File, io::Write},
};

impl From<std::io::Error> for ArbitrageError {
    fn from(error: std::io::Error) -> Self {
        ArbitrageError::PeristingError {
            msg: error.to_string(),
        }
    }
}

impl From<SerializerError> for ArbitrageError {
    fn from(error: SerializerError) -> Self {
        ArbitrageError::PeristingError {
            msg: error.to_string(),
        }
    }
}

#[derive(Deserialize)]
struct ArbitragePersisterConfig {
    file_name: String,
}

impl ArbitragePersisterConfig {
    pub fn new(json: Value) -> Self {
        serde_json::from_value(json).unwrap()
    }
}

#[derive(Debug)]
pub struct ArbitragePersister<T: Serializer> {
    file_handle: File,
    serializer: T,
    buffer: Vec<u8>,
}

impl<T: Serializer> ArbitrageExecutor for ArbitragePersister<T> {
    fn new(config: Value) -> Self {
        info!("Creating ArbitragePersister. Config: `{}`", config);
        let config = ArbitragePersisterConfig::new(config);

        let timestamp = Utc::now().format("%Y-%m-%dT%H%M%S").to_string();

        let file_name = format!("{}.{}", config.file_name, timestamp);
        info!("Creating the arbitrage file '{}'", file_name);
        let file_handle = File::create(&file_name).unwrap();
        Self {
            file_handle,
            serializer: T::new(),
            buffer: vec![0; BUFFER_SIZE],
        }
    }

    fn execute(&mut self, arbitrage: &ArbitrageMessage) -> Result<()> {
        let size = self
            .serializer
            .serialize_arbitrage(&arbitrage, &mut self.buffer)?;
        self.file_handle.write_all(&self.buffer[0..size])?;
        self.file_handle.write_all(&[EOL])?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use {
        super::ArbitragePersister,
        crate::arbitrage::{ArbitrageExecutor, ArbitrageMessage},
        common::{
            message::SequenceId,
            test_util::tests::{create_liquidity_pool, MockSerializer},
            AccountType, AmountType, LiquidityPoolState,
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
        let mut persister: ArbitragePersister<MockSerializer> = ArbitragePersister::new(config);

        let arbitrage = ArbitrageMessage::new(
            SequenceId::new(),
            1,
            &LiquidityPoolState::new(create_liquidity_pool(AccountType::OrcaWhirlpoolAccount)),
            &LiquidityPoolState::new(create_liquidity_pool(AccountType::RaydiumAmmPoolAccount)),
            67.89 as AmountType,
            12.13 as AmountType,
            14.15 as AmountType,
        );
        persister.execute(&arbitrage).unwrap();
        persister.execute(&arbitrage).unwrap();
    }
}
