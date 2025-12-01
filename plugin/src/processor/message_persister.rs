use {
    crate::processor::{Processor, ProcessorError, Result},
    chrono::Utc,
    common::{
        message::{AccountUpdateMessage, BlockUpdateMessage},
        read_from_file,
        serializer::{Serializer, SerializerError, BUFFER_SIZE},
        LiquidityPool, TokenAccount, EOL,
    },
    log::info,
    serde::{Deserialize, Serialize},
    std::{fs::File, io::Write},
};

impl From<std::io::Error> for ProcessorError {
    fn from(error: std::io::Error) -> Self {
        ProcessorError::IOError {
            msg: error.to_string(),
        }
    }
}

impl From<SerializerError> for ProcessorError {
    fn from(error: SerializerError) -> Self {
        ProcessorError::ProcessingError {
            msg: error.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct MessagePersisterConfig {
    data_directory: String,
}

impl MessagePersisterConfig {
    pub fn load(config_file_name: &str) -> Self {
        info!("Loading MessagePersister config `{}`", config_file_name);
        let config_file_contents = read_from_file(config_file_name);
        serde_json::from_str(&config_file_contents).unwrap()
    }
}

#[derive(Debug)]
pub struct MessagePersister<T: Serializer> {
    accounts_file_name: String,
    accounts_file_handle: File,
    messages_file_name: String,
    messages_file_handle: File,
    buffer: Vec<u8>,
    serializer: T,
}

impl<T: Serializer> MessagePersister<T> {
    pub fn accounts_file_name(&self) -> &str {
        &self.accounts_file_name
    }
    pub fn messages_file_name(&self) -> &str {
        &self.messages_file_name
    }
}

impl<T: Serializer> Processor for MessagePersister<T> {
    fn new(config_file_name: &str) -> Self
    where
        Self: Sized,
    {
        let config = MessagePersisterConfig::load(&config_file_name);

        let timestamp = Utc::now().format("%Y-%m-%dT%H%M%S").to_string();

        let accounts_file_name = format!("{}/accounts.{}", config.data_directory, timestamp);
        info!("Creating the accounts file '{}'", accounts_file_name);
        let accounts_file_handle = File::create(&accounts_file_name).unwrap();

        let messages_file_name = format!("{}/messages.{}", config.data_directory, timestamp);
        info!("Creating the messages file '{}'", accounts_file_name);
        let messages_file_handle = File::create(&messages_file_name).unwrap();
        Self {
            accounts_file_name,
            accounts_file_handle,
            messages_file_name,
            messages_file_handle,
            buffer: vec![0; BUFFER_SIZE],
            serializer: T::new(),
        }
    }

    fn register_token_account(&mut self, _msg: TokenAccount) -> Result<()> {
        // Noop for now
        Ok(())
    }

    fn update_liquidity_pool(&mut self, msg: LiquidityPool) -> Result<()> {
        let size = self.serializer.serialize_liquidity_pool(&msg, &mut self.buffer)?;
        self.accounts_file_handle.write_all(&mut self.buffer[0..size])?;
        self.accounts_file_handle.write_all(&[EOL])?;
        Ok(())
    }
    fn update_block(&mut self, msg: BlockUpdateMessage) -> Result<()> {
        let size = self.serializer.serialize_block_update(&msg, &mut self.buffer)?;
        self.messages_file_handle.write_all(&mut self.buffer[0..size])?;
        self.messages_file_handle.write_all(&[EOL])?;
        Ok(())
    }

    fn update_account(&mut self, msg: AccountUpdateMessage) -> Result<()> {
        let size = self.serializer.serialize_account_update(&msg, &mut self.buffer)?;
        self.messages_file_handle.write_all(&mut self.buffer[0..size])?;
        self.messages_file_handle.write_all(&[EOL])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        common::{
            message::{AccountUpdateMessage, BlockUpdateMessage},
            test_util::tests::{MOCK_wSOL_TOKEN, MockSerializer, MOCK_USDC_TOKEN},
            LiquidityGroupCode, LiquidityPool, OrcaWhirlpoolLp,
        },
        solana_sdk::{pubkey::Pubkey, signature::Signature},
        std::fs,
        tempfile::NamedTempFile,
    };

    fn populate_config_file(config_file: &mut NamedTempFile, data_directory: &str) {
        let config_str = format!(
            r#"
            {{
                "data_directory": "{}"
            }}"#,
            data_directory
        );

        fs::write(config_file.path(), config_str).unwrap();
    }

    #[test]
    fn test_config() {
        const DATA_DIRECTORY: &str = "Test Data directory.";
        let mut config_file = NamedTempFile::new().unwrap();
        populate_config_file(&mut config_file, DATA_DIRECTORY);
        let config = MessagePersisterConfig::load(config_file.path().to_str().unwrap());
        assert_eq!(config.data_directory, DATA_DIRECTORY);
    }

    #[test]
    fn test_persist_lp_configuration() {
        let temp_dir: String = std::env::temp_dir().to_str().unwrap().to_string();
        let mut config_file = NamedTempFile::new().unwrap();
        populate_config_file(&mut config_file, &temp_dir);
        let mut processor =
            MessagePersister::<MockSerializer>::new(config_file.path().to_str().unwrap());

        assert!(processor
            .update_liquidity_pool(LiquidityPool::OrcaWhirlpool(OrcaWhirlpoolLp {
                liquidity_group: LiquidityGroupCode::SOL_USD,
                name: format!("OrcaWhrilpool"),
                address: Pubkey::new_unique(),
                base_token: MOCK_wSOL_TOKEN,
                min_base_token_balance: 0.01,
                quote_token: MOCK_USDC_TOKEN,
                min_quote_token_balance: 1.0,
                tick: 1
            }))
            .is_ok());
    }

    #[test]
    fn test_process_account_update() {
        let temp_dir: String = std::env::temp_dir().to_str().unwrap().to_string();
        let mut config_file = NamedTempFile::new().unwrap();
        populate_config_file(&mut config_file, &temp_dir);
        let mut processor =
            MessagePersister::<MockSerializer>::new(config_file.path().to_str().unwrap());

        assert!(processor
            .update_account(AccountUpdateMessage {
                slot: 1,
                address: Pubkey::new_unique(),
                data: vec![0u8; 32],
                txn_signature: Some(Signature::new_unique()),
            })
            .is_ok());
    }

    #[test]
    fn test_process_block_update() {
        let temp_dir: String = std::env::temp_dir().to_str().unwrap().to_string();
        let mut config_file = NamedTempFile::new().unwrap();
        populate_config_file(&mut config_file, &temp_dir);
        let mut processor =
            MessagePersister::<MockSerializer>::new(config_file.path().to_str().unwrap());

        assert!(processor
            .update_block(BlockUpdateMessage {
                slot: 123,
                block_time: Some(456),
                block_height: Some(789)
            })
            .is_ok());
    }
}
