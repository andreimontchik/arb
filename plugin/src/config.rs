use {
    common::read_from_file, log::info, serde_derive::Deserialize, serde_json, std::fmt, thiserror::Error,
};

#[derive(Error, Debug, Deserialize)]
pub struct PluginConfig {
    pub processor: String,
    pub tokens: String,
    pub token_accounts: String,
    pub orca: String,
    pub raydium: String,
}

impl PluginConfig {
    pub fn load(config_file_name: &str) -> Self {
        info!("Loading the Plugin configuration from ({}).", config_file_name);
        let config_file_contents = read_from_file(config_file_name);
        serde_json::from_str(&config_file_contents).unwrap()
    }
}
impl fmt::Display for PluginConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use {super::*, std::fs, tempfile::NamedTempFile};

    #[test]
    fn test_plugin_config() {
        const PROCESSOR: &str = "Processor configuration";
        const ORCA: &str = "Orca configuration file.";
        const RAYDIUM: &str = "Raydium configuration file";
        const TOKENS: &str = "Tokens configuration file";
        const TOKEN_ACCOUNTS: &str = "Token Accounts configuration file";
        let config_str = format!(
            r#"
            {{
                "libpath": "DUMMY_PATH",
                "processor":"{}",
                "tokens" : "{}",
                "token_accounts" : "{}",
                "orca": "{}",
                "raydium": "{}"
            }}"#,
            PROCESSOR, TOKENS, TOKEN_ACCOUNTS, ORCA, RAYDIUM
        );

        let file = NamedTempFile::new().unwrap();
        let file_path = file.path();
        fs::write(file_path, config_str).unwrap();

        let config = PluginConfig::load(file_path.to_str().unwrap());
        assert_eq!(config.processor, PROCESSOR);
        assert_eq!(config.orca, ORCA);
        assert_eq!(config.raydium, RAYDIUM);
        assert_eq!(config.tokens, TOKENS);
        assert_eq!(config.token_accounts, TOKEN_ACCOUNTS);
    }

    #[test]
    fn test_empty_plugin_config() {
        let config_str = r#"
            {
                "libpath": "DUMMY_PATH",
                "processor": "",
                "tokens" : "DUMMY_PATH",
                "token_accounts" : "DUMMY_PATH",
                "orca": "",
                "raydium": ""
            }"#;
        let file = NamedTempFile::new().unwrap();
        let file_path = file.path();
        fs::write(file_path, config_str).unwrap();

        let config = PluginConfig::load(file_path.to_str().unwrap());
        assert!(config.processor.is_empty());
        assert!(config.orca.is_empty());
        assert!(config.raydium.is_empty());
    }
}
