use {
    crate::{read_from_file, AmountType},
    log::info,
    serde_derive::Deserialize,
};

#[derive(Deserialize, Debug)]
pub struct WalletConfig {
    pub keypair: String,
    pub token_accounts: String,
}

impl WalletConfig {
    pub fn load(config_file_name: &str) -> Self {
        info!("Lading the WalletConfig from ({config_file_name}).");
        let config_file_contents = read_from_file(config_file_name);
        serde_json::from_str(&config_file_contents).unwrap()
    }
}

#[derive(Deserialize, Debug)]
pub struct TokenAccountsConfig(pub Vec<TokenAccountConfig>);

impl TokenAccountsConfig {
    pub fn load(config_file_name: &str) -> Self {
        info!(
            "Loading Token accounts configuration from  ({})",
            config_file_name
        );
        let config_file_contents = read_from_file(config_file_name);
        serde_json::from_str(&config_file_contents).unwrap()
    }

    pub fn token_accounts(&self) -> &Vec<TokenAccountConfig> {
        return &self.0;
    }
}

#[derive(Deserialize, Debug)]
pub struct TokenAccountConfig {
    pub code: String,
    pub address: String,
    pub min_amount: AmountType,
}

#[cfg(test)]
mod tests {
    use {crate::config::wallet_config::WalletConfig, std::fs, tempfile::NamedTempFile};

    #[test]
    fn test_wallet_config() {
        const WALLET_KEYPAIR: &str = "Wallet Keypair";
        const TOKE_ACCOUNTS_CONFIG: &str = "Token Account Config file";

        let config_str = format!(
            r#"
            {{
                "keypair": "{}",
                "token_accounts": "{}"
            }}"#,
            WALLET_KEYPAIR, TOKE_ACCOUNTS_CONFIG
        );

        let file = NamedTempFile::new().unwrap();
        let file_path = file.path();
        fs::write(file_path, config_str).unwrap();

        let config = WalletConfig::load(file_path.to_str().unwrap());
        assert_eq!(config.keypair, WALLET_KEYPAIR);
        assert_eq!(config.token_accounts, TOKE_ACCOUNTS_CONFIG);
    }
}
