use {
    super::{get_token, to_pubkey, Result},
    crate::{
        read_from_file, AmountType, LiquidityGroupCode, LiquidityPool, RaydiumAmmLp, Token, TokenCode,
    },
    log::info,
    serde_derive::Deserialize,
    serde_json,
    std::collections::HashMap,
};

#[derive(Deserialize)]
pub struct RaydiumAmmConfig {
    pub amm_program_id: String,
    pub authority: String,
    pub min_base_token_balance: AmountType,
    pub min_quote_token_balance: AmountType,
    pub amm_pools: Vec<RaydiumAmmPoolConfig>,
}

impl RaydiumAmmConfig {
    pub fn load(config_file_name: &str) -> Self {
        info!("Loading Raydium AMMs from '{}'.", config_file_name);
        let config_file_contents = read_from_file(config_file_name);
        serde_json::from_str(&config_file_contents).unwrap()
    }
}

#[derive(Deserialize, Debug)]
pub struct RaydiumAmmPoolConfig {
    pub enabled: bool,
    pub liquidity_group: String,
    pub address: String,
    pub base_token: String,
    pub base_token_vault: String,
    pub quote_token: String,
    pub quote_token_vault: String,
}

pub fn to_raydium_amm(
    program_id: &str,
    authority: &str,
    min_base_token_balance: AmountType,
    min_quote_token_balance: AmountType,
    config: &RaydiumAmmPoolConfig,
    tokens: &HashMap<TokenCode, Token>,
) -> Result<LiquidityPool> {
    Ok(LiquidityPool::RaydiumAmm(RaydiumAmmLp {
        liquidity_group: LiquidityGroupCode::try_from(config.liquidity_group.as_str())?,
        program_id: to_pubkey(program_id)?,
        name: format!("RaydiumAmm({}-{})", config.base_token, config.quote_token),
        address: to_pubkey(&config.address)?,
        authority: to_pubkey(authority)?,
        base_token: get_token(&config.base_token, &tokens)?,
        base_token_vault: to_pubkey(&config.base_token_vault)?,
        min_base_token_balance,
        quote_token: get_token(&config.quote_token, &tokens)?,
        quote_token_vault: to_pubkey(&config.quote_token_vault)?,
        min_quote_token_balance,
    }))
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::test_util::tests::{create_mock_tokens_hashmap, create_raydium_amm_pool_config},
        solana_sdk::pubkey::Pubkey,
        std::fs,
        tempfile::NamedTempFile,
    };

    const PUBKEY1: &str = "TEST_PUBKEY1";
    const PUBKEY2: &str = "TEST_PUBKEY2";
    const PUBKEY3: &str = "TEST_PUBKEY3";
    const PUBKEY5: &str = "TEST_PUBKEY5";
    const PUBKEY6: &str = "TEST_PUBKEY6";
    const TOKEN1: &str = "TOKEN1";
    const TOKEN1_BALANCE: AmountType = 1.01;
    const MIN_TOKEN1_BALANCE: AmountType = 0.01;
    const TOKEN2: &str = "TOKEN2";
    const TOKEN2_BALANCE: AmountType = 2.02;
    const MIN_TOKEN2_BALANCE: AmountType = 0.02;

    #[test]
    fn test_raydium_config() {
        let config_str = format!(
            r#"
    {{  "amm_program_id": "{}",
        "authority": "{}",
        "min_base_token_balance": {},
        "min_quote_token_balance": {},
        "amm_pools": [{{
            "enabled" : true,
            "liquidity_group": "{:?}",
            "address": "{}",
            "base_token": "{}",
            "base_token_vault": "{}",
            "min_base_token_balance": {},
            "quote_token": "{}",
            "quote_token_vault": "{}",
            "min_quote_token_balance": {}
        }}] }}"#,
            PUBKEY1,
            PUBKEY2,
            MIN_TOKEN1_BALANCE,
            MIN_TOKEN2_BALANCE,
            LiquidityGroupCode::SOL_USD,
            PUBKEY3,
            TOKEN1,
            PUBKEY5,
            TOKEN1_BALANCE,
            TOKEN2,
            PUBKEY6,
            TOKEN2_BALANCE,
        );

        let file = NamedTempFile::new().unwrap();
        let file_path = file.path();
        fs::write(file_path, config_str).unwrap();

        let config = RaydiumAmmConfig::load(file_path.to_str().unwrap());
        assert_eq!(config.amm_pools.len(), 1);
        assert_eq!(config.amm_pools[0].enabled, true);
        assert_eq!(
            LiquidityGroupCode::try_from(config.amm_pools[0].liquidity_group.as_str()).unwrap(),
            LiquidityGroupCode::SOL_USD
        );
        assert_eq!(config.amm_program_id, PUBKEY1);
        assert_eq!(config.authority, PUBKEY2);
        assert_eq!(config.min_base_token_balance, MIN_TOKEN1_BALANCE);
        assert_eq!(config.min_quote_token_balance, MIN_TOKEN2_BALANCE);
        assert_eq!(config.amm_pools[0].address, PUBKEY3);
        assert_eq!(config.amm_pools[0].base_token, TOKEN1);
        assert_eq!(config.amm_pools[0].base_token_vault, PUBKEY5);
        assert_eq!(config.amm_pools[0].quote_token, TOKEN2);
        assert_eq!(config.amm_pools[0].quote_token_vault, PUBKEY6);
    }

    #[test]
    fn test_to_raydium_amm() {
        let program_id = Pubkey::new_unique().to_string();
        let authority = Pubkey::new_unique().to_string();
        for _ in 0..257 {
            let config = create_raydium_amm_pool_config(
                LiquidityGroupCode::SOL_USD,
                TokenCode::wSOL,
                TokenCode::USDC,
            );

            let lp = to_raydium_amm(
                &program_id,
                &authority,
                TOKEN1_BALANCE,
                TOKEN2_BALANCE,
                &config,
                &create_mock_tokens_hashmap(),
            )
            .unwrap();
            if let LiquidityPool::RaydiumAmm(RaydiumAmmLp {
                liquidity_group,
                address,
                base_token,
                quote_token,
                base_token_vault,
                quote_token_vault,
                ..
            }) = lp
            {
                assert_eq!(liquidity_group.to_string(), config.liquidity_group);
                assert_eq!(Pubkey::from(address).to_string(), config.address);
                assert_eq!(base_token.code().to_string(), config.base_token);
                assert_eq!(quote_token.code().to_string(), config.quote_token);
                assert_eq!(
                    Pubkey::from(base_token_vault).to_string(),
                    config.base_token_vault
                );
                assert_eq!(
                    Pubkey::from(quote_token_vault).to_string(),
                    config.quote_token_vault
                );
            } else {
                panic!("Unexpected Liquidity Pool {:?}!", lp);
            }
        }
    }
}
