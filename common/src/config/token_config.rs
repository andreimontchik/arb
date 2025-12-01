use {
    super::{to_pubkey, Result},
    crate::{read_from_file, Token, TokenCode, TokenDecimalsType},
    log::info,
    serde::Deserialize,
};

#[derive(Deserialize, Debug)]
pub struct TokensConfig(pub Vec<TokenConfig>);

impl TokensConfig {
    pub fn load(config_file_name: &str) -> Self {
        info!("Loading Tokens from ({})", config_file_name);
        let config_file_contents = read_from_file(config_file_name);
        serde_json::from_str(&config_file_contents).unwrap()
    }

    pub fn tokens(&self) -> &Vec<TokenConfig> {
        return &self.0;
    }
}

#[derive(Deserialize, Debug)]
pub struct TokenConfig {
    pub code: String,
    pub mint: String,
    pub decimals: TokenDecimalsType,
}

pub fn to_token(config: &TokenConfig) -> Result<Token> {
    Ok(Token {
        code: TokenCode::try_from(config.code.as_str())?,
        mint: to_pubkey(&config.mint)?,
        decimals: config.decimals,
    })
}

#[cfg(test)]
mod tests {
    use {
        super::{to_token, TokenConfig, TokensConfig},
        crate::test_util::tests::{MOCK_wSOL_TOKEN, MOCK_USDC_TOKEN},
        std::fs,
        tempfile::NamedTempFile,
    };

    #[test]
    fn test_token_config() {
        let config_str = format!(
            r#"[{{
                    "code" : "{}",
                    "mint": "{}",
                    "decimals": {}
                }},
                {{
                    "code" : "{}",
                    "mint": "{}",
                    "decimals": {}
                }}]"#,
            MOCK_wSOL_TOKEN.code().to_string(),
            MOCK_wSOL_TOKEN.mint(),
            MOCK_wSOL_TOKEN.decimals(),
            MOCK_USDC_TOKEN.code().to_string(),
            MOCK_USDC_TOKEN.mint(),
            MOCK_USDC_TOKEN.decimals()
        );

        let file = NamedTempFile::new().unwrap();
        let file_path = file.path();
        fs::write(file_path, config_str).unwrap();

        let config = TokensConfig::load(file_path.to_str().unwrap());
        assert_eq!(config.tokens().len(), 2);
        assert_eq!(config.tokens()[0].code, MOCK_wSOL_TOKEN.code().to_string());
        assert_eq!(config.tokens()[0].mint, MOCK_wSOL_TOKEN.mint().to_string());
        assert_eq!(config.tokens()[0].decimals, MOCK_wSOL_TOKEN.decimals());
        assert_eq!(config.tokens()[1].code, MOCK_USDC_TOKEN.code().to_string());
        assert_eq!(config.tokens()[1].mint, MOCK_USDC_TOKEN.mint().to_string());
        assert_eq!(config.tokens()[1].decimals, MOCK_USDC_TOKEN.decimals());
    }

    #[test]
    fn test_to_token() {
        let token_config = TokenConfig {
            code: MOCK_USDC_TOKEN.code().to_string(),
            mint: MOCK_USDC_TOKEN.mint().to_string(),
            decimals: MOCK_USDC_TOKEN.decimals(),
        };

        let token = to_token(&token_config).unwrap();
        assert_eq!(token, MOCK_USDC_TOKEN);
    }
}
