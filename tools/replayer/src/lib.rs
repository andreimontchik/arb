use {
    common::{
        message::{AccountUpdateMessage, BlockUpdateMessage, Message},
        read_from_file,
        serializer::{CsvSerializer, Serializer},
        AmountType, Token, TokenAccount, TokenCode, TokenDigitsType,
    },
    plugin::processor::Processor,
    serde_derive::Deserialize,
    serde_json,
    solana_sdk::{clock::Slot, program_option::COption, program_pack::Pack, signature::Signature},
    spl_token::state::{Account, AccountState},
    std::{
        collections::HashMap,
        fs::File,
        io::{BufRead, BufReader},
        time::{SystemTime, UNIX_EPOCH},
    },
};

#[derive(Deserialize)]
pub struct ReplayerConfig {
    pub tokens: String,
    pub token_accounts: String,
    pub token_funding: Vec<TokenFundingConfig>,
    pub processor: String,
    pub orca: String,
    pub raydium: String,
}

impl ReplayerConfig {
    pub fn load(config_file_name: &str) -> Self {
        let config_file_contents = read_from_file(config_file_name);
        serde_json::from_str(&config_file_contents).unwrap()
    }
}

#[derive(Deserialize)]
pub struct TokenFundingConfig {
    pub code: String,
    pub amount: AmountType,
}

pub fn process(mut processor: impl Processor, file_name: &str) -> (u32, u32) {
    println!("Processing the messages file ({})", file_name);

    let file_reader = BufReader::new(File::open(file_name).unwrap());

    let mut serialier = CsvSerializer::new();

    let mut processed_counter: u32 = 0;
    let mut failed_counter: u32 = 0;

    let mut last_slot: Slot = 0;
    for line in file_reader.lines() {
        let line = line.unwrap();
        let msg = serialier.deserialize_message(line.as_bytes()).unwrap();

        let block_update: Option<Message> =
            if let Message::AccountUpdate(AccountUpdateMessage { slot, .. }) = msg {
                // Create the BlockUpdate message for every new slot.
                let result = if slot > last_slot {
                    Some(Message::BlockUpdate(BlockUpdateMessage {
                        slot,
                        block_time: Some(
                            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
                        ),
                        block_height: Some(slot),
                    }))
                } else {
                    None
                };
                last_slot = slot;
                result
            } else {
                None
            };

        if let Err(_) = processor.process(msg) {
            failed_counter += 1;
        }

        if let Some(block_update) = block_update {
            let _ = processor.process(block_update);
        }

        processed_counter += 1;
        if (processed_counter % 1000) == 0 {
            println!("Processed {} rows...", processed_counter)
        }
    }

    (processed_counter, failed_counter)
}

pub fn fund_token_accounts(
    tokens: &HashMap<TokenCode, Token>,
    token_accounts: &HashMap<TokenCode, TokenAccount>,
    token_funding_config: Vec<TokenFundingConfig>,
    processor: &mut impl Processor,
) {
    for token_funding in token_funding_config {
        if token_funding.amount > 0.0 {
            println!(
                "Funding Token Account ({}) with ({})",
                token_funding.code, token_funding.amount
            );
            let token_code = TokenCode::try_from(&token_funding.code).unwrap();
            let token = tokens
                .get(&token_code)
                .expect(&format!("Unsupported funding for ({:?})!", token_code));
            let token_account = token_accounts
                .get(&token_code)
                .expect(&format!("Unsupported token account for ({:?})!", token_code));

            let account = Account {
                mint: *token.mint(),
                owner: *token_account.address(),
                amount: (token_funding.amount * 10f64.powi(token.decimals().into())) as TokenDigitsType,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            };

            let mut data: Vec<u8> = vec![0; Account::LEN];
            Account::pack(account, &mut data)
                .expect(&format!("Failed to pack the Account ({:?})", account));

            let msg = Message::AccountUpdate(AccountUpdateMessage {
                slot: 1,
                address: *token_account.address(),
                data,
                txn_signature: (Some(Signature::new_unique())),
            });

            processor.process(msg).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use {super::*, std::fs, tempfile::NamedTempFile};

    #[test]
    fn test_replayer_config() {
        const TOKENS: &str = "Tokens configuration file";
        const TOKEN_ACCOUNTS: &str = "Tokens accounts configuration file";
        const PROCESSOR: &str = "Processor configuration";
        const ORCA: &str = "Orca configuration file.";
        const RAYDIUM: &str = "Orca configuration file.";
        let config_str = format!(
            r#"
            {{
                "tokens": "{}",
                "token_accounts": "{}",
                "token_funding": [
                    {{
                        "code": "wSOL",
                        "amount": 5.0
                    }},
                    {{
                        "code": "USDC",
                        "amount": 500.0
                    }},
                    {{
                        "code": "USDT",
                        "amount": 500.0
                    }}
                ],                
                "processor": "{}",
                "orca": "{}",
                "raydium": "{}"
            }}"#,
            TOKENS, TOKEN_ACCOUNTS, PROCESSOR, ORCA, RAYDIUM
        );

        let file = NamedTempFile::new().unwrap();
        let file_path = file.path();
        fs::write(file_path, config_str).unwrap();

        let config = ReplayerConfig::load(file_path.to_str().unwrap());
        assert_eq!(config.tokens, TOKENS);
        assert_eq!(config.processor, PROCESSOR);
        assert_eq!(config.orca, ORCA);
        assert_eq!(config.raydium, RAYDIUM);
    }
}
