mod orca_config;
mod raydium_config;
mod token_config;
mod wallet_config;

use {
    crate::{CommonError, Token, TokenCode},
    anyhow::Result,
    solana_sdk::pubkey::Pubkey,
    std::{collections::HashMap, str::FromStr},
};
pub use {
    orca_config::{to_orca_whirlpool, OrcaConfig, OrcaWhirlpoolConfig},
    raydium_config::{to_raydium_amm, RaydiumAmmConfig, RaydiumAmmPoolConfig},
    token_config::{to_token, TokenConfig, TokensConfig},
    wallet_config::{TokenAccountConfig, TokenAccountsConfig, WalletConfig},
};

pub fn to_pubkey(pubkey_str: &str) -> Result<Pubkey> {
    Ok(Pubkey::from_str(pubkey_str)?)
}

pub fn get_token(code_str: &str, tokens: &HashMap<TokenCode, Token>) -> Result<Token> {
    let token_code = TokenCode::try_from(code_str)?;
    let result = tokens
        .get(&token_code)
        .ok_or(CommonError::InvalidTokenCode { code: token_code })?;
    Ok(*result)
}

#[cfg(test)]
mod tests {
    use {
        crate::{
            config::{get_token, to_pubkey},
            test_util::tests::{
                create_mock_tokens_hashmap, MOCK_wSOL_TOKEN, MOCK_USDC_TOKEN, MOCK_USDT_TOKEN,
            },
        },
        solana_sdk::pubkey::Pubkey,
    };

    #[test]
    fn test_to_pubkey() {
        for _ in 0..257 {
            let src_pk = Pubkey::new_unique();
            let res_pk = to_pubkey(&src_pk.to_string()).unwrap();
            assert_eq!(src_pk, res_pk);
        }
    }

    #[test]
    fn test_get_token() {
        let tokens = create_mock_tokens_hashmap();
        assert_eq!(
            get_token(&MOCK_wSOL_TOKEN.code().to_string(), &tokens).unwrap(),
            MOCK_wSOL_TOKEN
        );
        assert_eq!(
            get_token(&MOCK_USDC_TOKEN.code().to_string(), &tokens).unwrap(),
            MOCK_USDC_TOKEN
        );
        assert_eq!(
            get_token(&MOCK_USDT_TOKEN.code().to_string(), &tokens).unwrap(),
            MOCK_USDT_TOKEN
        );
    }
}
