use {
    super::{get_token, to_pubkey, Result},
    crate::{
        read_from_file, AmountType, LiquidityGroupCode, LiquidityPool, OrcaWhirlpoolLp, TickType, Token,
        TokenCode,
    },
    log::info,
    serde_derive::Deserialize,
    serde_json,
    std::collections::HashMap,
};

#[derive(Deserialize)]
pub struct OrcaConfig {
    pub whirlpools: Vec<OrcaWhirlpoolConfig>,
}

impl OrcaConfig {
    pub fn load(config_file_name: &str) -> Self {
        info!("Loading Orca configuration from ({}).", config_file_name);
        let config_file_contents = read_from_file(config_file_name);
        serde_json::from_str(&config_file_contents).unwrap()
    }
}

#[derive(Deserialize, Debug)]
pub struct OrcaWhirlpoolConfig {
    pub enabled: bool,
    pub liquidity_group: String,
    pub pubkey: String,
    pub token_a: String,
    pub min_token_a_balance: AmountType,
    pub token_b: String,
    pub min_token_b_balance: AmountType,
    pub tick: TickType,
}

pub fn to_orca_whirlpool(
    config: &OrcaWhirlpoolConfig,
    tokens: &HashMap<TokenCode, Token>,
) -> Result<LiquidityPool> {
    Ok(LiquidityPool::OrcaWhirlpool(OrcaWhirlpoolLp {
        liquidity_group: LiquidityGroupCode::try_from(config.liquidity_group.as_str())?,
        name: format!(
            "OrcaWhirlpool({}-{}-{})",
            config.token_a, config.token_b, config.tick
        ),
        address: to_pubkey(&config.pubkey)?,
        base_token: get_token(&config.token_a, &tokens)?,
        min_base_token_balance: config.min_token_a_balance,
        quote_token: get_token(&config.token_b, &tokens)?,
        min_quote_token_balance: config.min_token_b_balance,
        tick: config.tick,
    }))
}

#[cfg(test)]
mod tests {
    use {
        super::{to_orca_whirlpool, OrcaWhirlpoolConfig},
        crate::{
            config::orca_config::OrcaConfig, test_util::tests::create_mock_tokens_hashmap, AmountType,
            LiquidityGroupCode, LiquidityPool, OrcaWhirlpoolLp, TickType, TokenCode,
        },
        solana_sdk::pubkey::Pubkey,
        std::fs,
        tempfile::NamedTempFile,
    };

    const PUBKEY1: &str = "TEST_PUBKEY1";
    const PUBKEY2: &str = "TEST_PUBKEY2";

    #[test]
    fn test_orca_config() {
        const TOKEN_A1: &str = "TOKEN_A1";
        const TOKEN_A1_BALANCE: AmountType = 0.01;
        const TOKEN_B1: &str = "TOKEN_B1";
        const TOKEN_B1_BALANCE: AmountType = 1.01;
        const TICK1: u8 = 1;
        const TOKEN_A2: &str = "TOKEN_A2";
        const TOKEN_A2_BALANCE: AmountType = 0.02;
        const TOKEN_B2: &str = "TOKEN_B2";
        const TOKEN_B2_BALANCE: AmountType = 2.02;
        const TICK2: TickType = 2;

        let config_str = format!(
            r#"{{
                "whirlpools": [{{
                    "enabled" : true,
                    "liquidity_group": "{:?}",
                    "pubkey": "{}",
                    "token_a": "{}",
                    "min_token_a_balance": {},
                    "token_b": "{}",
                    "min_token_b_balance": {},
                    "tick": {}
                }},
                {{
                    "enabled" : true,
                    "liquidity_group": "{:?}",
                    "pubkey": "{}",
                    "token_a": "{}",
                    "min_token_a_balance": {},
                    "token_b": "{}",
                    "min_token_b_balance": {},
                    "tick": {}
            }}] }}"#,
            LiquidityGroupCode::SOL_USD,
            PUBKEY1,
            TOKEN_A1,
            TOKEN_A1_BALANCE,
            TOKEN_B1,
            TOKEN_B1_BALANCE,
            TICK1,
            LiquidityGroupCode::SOL_USD,
            PUBKEY2,
            TOKEN_A2,
            TOKEN_A2_BALANCE,
            TOKEN_B2,
            TOKEN_B2_BALANCE,
            TICK2
        );

        let file = NamedTempFile::new().unwrap();
        let file_path = file.path();
        fs::write(file_path, config_str).unwrap();

        let config = OrcaConfig::load(file_path.to_str().unwrap());
        assert_eq!(config.whirlpools.len(), 2);
        assert_eq!(config.whirlpools[0].pubkey, PUBKEY1);
        assert_eq!(config.whirlpools[0].token_a, TOKEN_A1);
        assert_eq!(config.whirlpools[0].token_b, TOKEN_B1);
        assert_eq!(config.whirlpools[0].tick, TICK1);
        assert_eq!(config.whirlpools[1].pubkey, PUBKEY2);
        assert_eq!(config.whirlpools[1].token_a, TOKEN_A2);
        assert_eq!(config.whirlpools[1].token_b, TOKEN_B2);
        assert_eq!(config.whirlpools[1].tick, TICK2);
    }

    #[test]
    fn test_empty_orca_config() {
        let config_str = "{ \"whirlpool_program_id\": \"\", \"whirlpools\": [] }";
        let file = NamedTempFile::new().unwrap();
        let file_path = file.path();
        fs::write(file_path, config_str).unwrap();

        let config = OrcaConfig::load(file_path.to_str().unwrap());
        assert_eq!(config.whirlpools.len(), 0);
    }

    #[test]
    fn test_to_orca_whirlpool() {
        for i in 0..255 {
            let config = OrcaWhirlpoolConfig {
                enabled: true,
                liquidity_group: LiquidityGroupCode::SOL_USD.to_string(),
                pubkey: Pubkey::new_unique().to_string(),
                token_a: TokenCode::wSOL.to_string(),
                min_token_a_balance: 0.001,
                token_b: TokenCode::USDC.to_string(),
                min_token_b_balance: 1.0,
                tick: i,
            };

            let lp = to_orca_whirlpool(&config, &create_mock_tokens_hashmap()).unwrap();
            if let LiquidityPool::OrcaWhirlpool(OrcaWhirlpoolLp {
                liquidity_group,
                address,
                base_token,
                quote_token,
                tick,
                ..
            }) = lp
            {
                assert_eq!(liquidity_group.to_string(), config.liquidity_group);
                assert_eq!(Pubkey::from(address).to_string(), config.pubkey);
                assert_eq!(base_token.code().to_string(), config.token_a);
                assert_eq!(quote_token.code().to_string(), config.token_b);
                assert_eq!(tick, config.tick);
            } else {
                panic!("Unexpected Liquidity Pool {:?}!", lp);
            }
        }
    }
}
