#[cfg(test)]
pub mod tests {
    use {
        common::{
            message::{AccountUpdateMessage, BlockUpdateMessage, Message},
            serializer::{CsvSerializer, Serializer},
            test_util::tests::create_liquidity_pool,
            AccountType, LiquidityPool,
        },
        plugin::processor::{MessagePersister, Processor},
        solana_sdk::{
            clock::{Slot, UnixTimestamp},
            pubkey::Pubkey,
            signature::Signature,
        },
        std::{
            fs::{self, File},
            io::{BufRead, BufReader},
        },
        tempfile::NamedTempFile,
    };

    fn create_processor() -> MessagePersister<CsvSerializer> {
        let temp_dir: String = std::env::temp_dir().to_str().unwrap().to_string();
        let config_file = NamedTempFile::new().unwrap();
        let config_str = format!(
            r#"
        {{
            "data_directory": "{}"
        }}"#,
            temp_dir
        );

        fs::write(config_file.path(), config_str).unwrap();
        MessagePersister::<CsvSerializer>::new(config_file.path().to_str().unwrap())
    }

    const TOTAL_MSG: u16 = 513;

    #[test]
    fn test_pesist_lp_configuration() {
        let mut processor = create_processor();
        let messages: Vec<LiquidityPool> = (0..TOTAL_MSG)
            .map(|i| match i % 2 {
                0 => create_liquidity_pool(AccountType::OrcaWhirlpoolAccount),
                _ => create_liquidity_pool(AccountType::RaydiumAmmPoolAccount),
            })
            .collect();

        // Save
        for msg in messages {
            assert!(processor.update_liquidity_pool(msg).is_ok());
        }
    }

    #[test]
    fn test_persist_messages() {
        let mut processor = create_processor();

        let mut src_msg = Vec::<Message>::new();
        for i in 0..TOTAL_MSG {
            let slot = i as Slot;
            if (i % 3) == 0 {
                let address = Pubkey::new_unique();
                let data = vec![i as u8; i as usize];
                let txn_signature = if i % 2 == 0 {
                    Some(Signature::new_unique())
                } else {
                    None
                };
                let msg = AccountUpdateMessage {
                    slot,
                    address,
                    data: data.clone(),
                    txn_signature,
                };
                assert!(processor.update_account(msg).is_ok());
                src_msg.push(Message::AccountUpdate(AccountUpdateMessage {
                    slot,
                    address,
                    data,
                    txn_signature,
                }));
            } else {
                let block_time = if i % 2 == 0 {
                    Some(i as UnixTimestamp)
                } else {
                    None
                };
                let block_height = if i % 6 == 0 { Some(i as u64) } else { None };
                let msg = BlockUpdateMessage {
                    slot,
                    block_time,
                    block_height,
                };
                assert!(processor.update_block(msg).is_ok());
                src_msg.push(Message::BlockUpdate(BlockUpdateMessage {
                    slot,
                    block_time,
                    block_height,
                }));
            }
        }

        // Read
        let mut serialier = CsvSerializer::new();

        let file_reader = BufReader::new(File::open(processor.messages_file_name()).unwrap());
        for (i, line_r) in file_reader.lines().enumerate() {
            let line = line_r.unwrap();
            let dec_msg = serialier.deserialize_message(line.as_bytes()).unwrap();
            assert_eq!(src_msg[i], dec_msg);
        }
    }
}
