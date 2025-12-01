pub mod config;
pub mod message;
pub mod serializer;
pub mod test_util;
pub mod metrics;

use {
    crate::config::{to_pubkey, WalletConfig}, anyhow::{bail, Result}, config::{TokenAccountConfig, TokenAccountsConfig}, solana_sdk::{
        clock::Slot, pubkey::Pubkey, signature::{Keypair, Signature}, signer::EncodableKey
    }, std::{collections::HashMap, fs::File, io::Read}, thiserror::Error
};


#[derive(Error, Debug)]
pub enum CommonError {
    #[error("({code})")]
    InvalidAccountType { code: String },
    #[error("({code})")]
    InvalidToken { code: String },
    #[error("({code})")]
    InvalidLiquidityGroup { code: String },
    #[error("{address}")]
    InvalidLiquidityPool { address: Pubkey },
    #[error("({path}, {error})")]
    InvalidKeypairFile { path: String, error: String },
    #[error("{}", code.to_string())]
    InvalidTokenCode { code: TokenCode },
    #[error("{error}")]
    InvalidState { error: String },
    #[error("{}", code.to_string())]
    InvalidSwapType { code: String },
    #[error("({src}) <- {error}")]
    InvalidMessageSource { src: String, error: String },
    #[error("{error}")]
    CalculationError { error: String },
}

pub type AddressType = [u8; 32];
pub type TickType = u8;
pub type TokenDigitsType = u64;
pub type TokenDecimalsType = u8;

//TODO: replace with AmountType
pub type PriceType = f64;
pub type AmountType = f64;
pub type DecimalPercentageType = f64;

pub const ONE_BP_DECIMAL: DecimalPercentageType = 0.0001;
pub const ONE_HUNDRED_OF_BP_DECIMAL: DecimalPercentageType = 0.000001;

pub const EOL: u8 = '\n' as u8;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccountType {
    TokenAccount = 10,
    OrcaWhirlpoolProgramId = 100,
    OrcaWhirlpoolAccount = 101,
    RaydiumAmmPoolProgramId = 200,
    RaydiumAmmPoolAccount = 201,
    RaydiumAmmPoolVaultForBaseToken = 202,
    RaydiumAmmPoolVaultForQuoteToken = 203,
    _Saber = 300,
}

impl TryFrom<&str> for AccountType {
    type Error = anyhow::Error;
    fn try_from(code: &str) -> Result<Self> {
        match code {
            "OrcaWhirlpoolProgramId" => Ok(AccountType::OrcaWhirlpoolProgramId),
            "OrcaWhirlpoolAccount" => Ok(AccountType::OrcaWhirlpoolAccount),
            "RaydiumAmmPoolProgramId" => Ok(AccountType::RaydiumAmmPoolProgramId),
            "RaydiumAmmPoolAccount" => Ok(AccountType::RaydiumAmmPoolAccount),
            "RaydiumAmmPoolVaultForBaseToken" => Ok(AccountType::RaydiumAmmPoolVaultForBaseToken),
            "RaydiumAmmPoolVaultForQuoteToken" => Ok(AccountType::RaydiumAmmPoolVaultForQuoteToken),
            _ => bail!(CommonError::InvalidAccountType {                code: code.to_string()}),
        }
    }
}

#[derive(Debug)]
pub enum Side {
    Base,
    Quote,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum TokenCode {
    wSOL,
    bSOL,
    mSOL,
    stSOL,
    USDC,
    USDT,
}

impl TryFrom<&str> for TokenCode {
    type Error = anyhow::Error;
    fn try_from(code: &str) -> Result<Self> {
        match code {
            "wSOL" => Ok(TokenCode::wSOL),
            "bSOL" => Ok(TokenCode::bSOL),
            "mSOL" => Ok(TokenCode::mSOL),
            "stSOL" => Ok(TokenCode::stSOL),
            "USDC" => Ok(TokenCode::USDC),
            "USDT" => Ok(TokenCode::USDT),
            _ => bail!(CommonError::InvalidToken {
                code: code.to_string(),
            }),
        }
    }
}

impl TryFrom<&String> for TokenCode {
    type Error = anyhow::Error;
    fn try_from(code: &String) -> Result<Self> {
        Self::try_from(code.as_str())
    }
}


impl ToString for TokenCode {
    fn to_string(&self) -> String {
        match self {
            TokenCode::wSOL => "wSOL".to_string(),
            TokenCode::bSOL => "bSOL".to_string(),
            TokenCode::mSOL => "mSOL".to_string(),
            TokenCode::stSOL => "stSOL".to_string(),
            TokenCode::USDC => "USDC".to_string(),
            TokenCode::USDT => "USDT".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Token {
    code: TokenCode,
    mint: Pubkey,
    decimals: TokenDecimalsType,
}

impl Token {
    pub fn code(&self) -> TokenCode {
        self.code
    }

    pub fn mint(&self) -> &Pubkey {
        &self.mint
    }

    pub fn decimals(&self) -> TokenDecimalsType {
        self.decimals
    }
}

#[derive(Debug, Clone)]
pub struct TokenAccount {
    code: TokenCode,
    address: Pubkey,
    min_balance: AmountType,
}

impl TokenAccount {
    pub fn new(code: TokenCode, address: Pubkey, min_balance: AmountType) -> Self {
        TokenAccount {
            code,
            address,
            min_balance,
        }
    }

    pub fn code(&self) -> TokenCode {
        self.code
    }

    pub fn address(&self) -> &Pubkey {
        &self.address
    }
    pub fn min_balance(&self) -> AmountType {
        self.min_balance
    }
}

impl TryFrom<&TokenAccountConfig> for TokenAccount {
    type Error = anyhow::Error;

    fn try_from(config: &TokenAccountConfig) -> Result<Self> {
        let code = TokenCode::try_from(&config.code)?;
        let address = to_pubkey(&config.address)?;        
        Ok(TokenAccount {
            code,
            address,
            min_balance: config.min_amount,
        })
    }
}

#[derive(Debug)]
pub struct Wallet {
    keypair: Keypair,
    token_accounts: HashMap<TokenCode, TokenAccount>,
}

impl Wallet {
    pub fn new(config: &WalletConfig) -> Result<Self> {
        let wallet_keypair = Keypair::read_from_file(&config.keypair).map_err(|error| {
            CommonError::InvalidKeypairFile {
                path: config.keypair.clone(),
                error: error.to_string(),
            }
        })?;

        let token_accounts_config = TokenAccountsConfig::load(&config.token_accounts);
        
        let mut token_accounts: HashMap<TokenCode, TokenAccount> = HashMap::new();
        for token_account_config in token_accounts_config.token_accounts() {
            let code = TokenCode::try_from(token_account_config.code.as_str())?;
            token_accounts.insert(
                code.clone(),
                TokenAccount {
                    code,
                    address: to_pubkey(&token_account_config.address)?,
                    min_balance: token_account_config.min_amount,
                },
            );
        }

        Ok(Wallet {
            keypair: wallet_keypair,
            token_accounts,
        })
    }

    pub fn keypair(&self) -> &Keypair {
        &self.keypair
    }

    pub fn token_account(&self, token_code: &TokenCode) -> Result<&TokenAccount> {
        Ok(self.token_accounts.get(token_code).ok_or(CommonError::InvalidTokenCode {
            code: token_code.clone(),
        })?)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum LiquidityGroupCode {
    SOL_USD,
}

impl TryFrom<&str> for LiquidityGroupCode {
    type Error = anyhow::Error;
    fn try_from(code: &str) -> Result<Self> {
        match code {
            "SOL_USD" => Ok(LiquidityGroupCode::SOL_USD),
            _ => bail!(CommonError::InvalidLiquidityGroup {
                code: code.to_string(),
            }),
        }
    }
}

impl ToString for LiquidityGroupCode {
    fn to_string(&self) -> String {
        match self {
            LiquidityGroupCode::SOL_USD => "SOL_USD".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiquidityPool {
    RaydiumAmm(RaydiumAmmLp),
    OrcaWhirlpool(OrcaWhirlpoolLp),
}

impl LiquidityPool {
    pub fn pool_type(lp: &LiquidityPool) -> &AccountType {
        match lp {
            LiquidityPool::RaydiumAmm { .. } => &AccountType::RaydiumAmmPoolAccount,
            LiquidityPool::OrcaWhirlpool { .. } => &AccountType::OrcaWhirlpoolAccount,
        }
    }

    pub fn liquidity_group(lp: &LiquidityPool) -> LiquidityGroupCode {
        match lp {
            LiquidityPool::RaydiumAmm(RaydiumAmmLp { liquidity_group, .. }) => *liquidity_group,
            LiquidityPool::OrcaWhirlpool(OrcaWhirlpoolLp { liquidity_group, .. }) => *liquidity_group,
        }
    }

    pub fn program_id(lp: &LiquidityPool) -> &Pubkey {
        match lp {
            LiquidityPool::RaydiumAmm(lp) => &lp.program_id,
            LiquidityPool::OrcaWhirlpool(_) => unimplemented!(),
        }
    }

    pub fn name(lp: &LiquidityPool) -> &str {
        match lp {
            LiquidityPool::RaydiumAmm(RaydiumAmmLp { name, .. }) => name,
            LiquidityPool::OrcaWhirlpool(OrcaWhirlpoolLp { name, .. }) => name,
        }
    }

    pub fn address(lp: &LiquidityPool) -> &Pubkey {
        match lp {
            LiquidityPool::RaydiumAmm(RaydiumAmmLp { address, .. }) => address,
            LiquidityPool::OrcaWhirlpool(OrcaWhirlpoolLp { address, .. }) => address,
        }
    }

    pub fn accounts(lp: &LiquidityPool) -> HashMap<Pubkey, AccountType> {
        let mut result = HashMap::new();
        match lp {
            LiquidityPool::RaydiumAmm(RaydiumAmmLp {
                address,
                base_token_vault,
                quote_token_vault,
                ..
            }) => {
                result.insert(*address, AccountType::RaydiumAmmPoolAccount);
                result.insert(*base_token_vault, AccountType::RaydiumAmmPoolVaultForBaseToken);
                result.insert(*quote_token_vault, AccountType::RaydiumAmmPoolVaultForQuoteToken);
            }
            LiquidityPool::OrcaWhirlpool(OrcaWhirlpoolLp { address, .. }) => {
                result.insert(*address, AccountType::OrcaWhirlpoolAccount);
            }
        }
        result
    }

    pub fn base_token(lp: &LiquidityPool) -> &Token {
        match lp {
            LiquidityPool::RaydiumAmm(lp) => &lp.base_token,
            LiquidityPool::OrcaWhirlpool(lp) => &lp.base_token,
        }
    }

    pub fn min_base_token_balance(lp: &LiquidityPool) -> AmountType {
        match lp {
            LiquidityPool::RaydiumAmm(lp) => lp.min_base_token_balance,
            LiquidityPool::OrcaWhirlpool(lp) => lp.min_base_token_balance,
        }
    }

    pub fn quote_token(lp: &LiquidityPool) -> &Token {
        match lp {
            LiquidityPool::RaydiumAmm(lp) => &lp.quote_token,
            LiquidityPool::OrcaWhirlpool(lp) => &lp.quote_token,
        }
    }

    pub fn min_quote_token_balance(lp: &LiquidityPool) -> AmountType {
        match lp {
            LiquidityPool::RaydiumAmm(lp) => lp.min_quote_token_balance,
            LiquidityPool::OrcaWhirlpool(lp) => lp.min_quote_token_balance,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RaydiumAmmLp {
    pub liquidity_group: LiquidityGroupCode,
    pub program_id: Pubkey,
    pub name: String,
    pub address: Pubkey,
    pub authority: Pubkey,
    pub base_token: Token,
    pub base_token_vault: Pubkey,
    // TODO: move to LG config
    pub min_base_token_balance: AmountType,
    pub quote_token: Token,
    pub quote_token_vault: Pubkey,
    // TODO: move to LG config
    pub min_quote_token_balance: AmountType,
}

impl RaydiumAmmLp {
    pub fn calc_quote_token_amount_to_buy(current_base_token_amount: AmountType, current_quote_token_amount: AmountType, fee: DecimalPercentageType,base_token_amount_to_buy: AmountType) -> AmountType {
        if current_base_token_amount <= base_token_amount_to_buy {
           return AmountType::NAN 
        }

        let new_base_token_amount = current_base_token_amount - base_token_amount_to_buy;
        let new_quote_token_amount = current_base_token_amount * current_quote_token_amount / new_base_token_amount;

        // Apply fee
        let mut result = new_quote_token_amount - current_quote_token_amount;
        result = result / ( 1.0 - fee);

        result
    }

    pub fn calc_quote_token_amount_for_selling(current_base_token_amount: AmountType, current_quote_token_amount: AmountType, fee: DecimalPercentageType,base_token_amount_to_sell: AmountType) -> AmountType {
        //Apply fee
        let base_token_amount_to_sell_without_fee = base_token_amount_to_sell * ( 1.0 - fee);
        
        let new_base_token_amount = current_base_token_amount + base_token_amount_to_sell_without_fee;
        let new_quote_token_amount = current_base_token_amount * current_quote_token_amount / new_base_token_amount;

        if current_quote_token_amount <= new_quote_token_amount {
            return AmountType::NAN 
         }

         current_quote_token_amount - new_quote_token_amount
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct OrcaWhirlpoolLp {
    pub liquidity_group: LiquidityGroupCode,
    pub name: String,
    pub address: Pubkey,
    pub base_token: Token,
    pub min_base_token_balance: AmountType,
    pub quote_token: Token,
    pub min_quote_token_balance: AmountType,
    pub tick: TickType,
}

impl OrcaWhirlpoolLp {
    pub fn calc_quote_token_amount_to_buy(_current_base_token_amount: AmountType, _current_quote_token_amount: AmountType, _fee: DecimalPercentageType,_base_token_amount_to_buy: AmountType) -> AmountType {
        unimplemented!()
    }
    pub fn calc_quote_token_amount_for_selling(_current_base_token_amount: AmountType, _current_quote_token_amount: AmountType, _fee: DecimalPercentageType,_base_token_amount_to_sell: AmountType) -> AmountType {
        unimplemented!()
    }
}


#[derive(Debug, PartialEq)]
pub struct LiquidityPoolState {
    pub liquidity_pool: LiquidityPool,
    pub base_token_digits: TokenDigitsType,
    pub quote_token_digits: TokenDigitsType,
    pub fee_ratio: DecimalPercentageType,
    pub healthy: bool,
    pub latest_upd_slot: Slot,
    pub latest_upd_txn_sig: Option<Signature>,
}

impl LiquidityPoolState {
    pub fn new(lp: LiquidityPool) -> Self {
        Self {
            liquidity_pool: lp,
            base_token_digits: 0,
            quote_token_digits: 0,
            fee_ratio: f64::NAN,
            healthy: false,
            latest_upd_slot: 0,
            latest_upd_txn_sig: None,
        }
    }

    pub fn liquidity_group(&self) -> LiquidityGroupCode {
        LiquidityPool::liquidity_group(&self.liquidity_pool)
    }

    pub fn is_computable(&self) -> bool {
        self.healthy        
            && self.base_token_digits > 0
            && self.quote_token_digits > 0
            && self.fee_ratio.is_finite()
            && ! self.is_low_base_token_balance()
            && ! self.is_low_quote_token_balance()
            // To avoid using stale lps.
            && self.latest_upd_txn_sig.is_some()
    }

    // Base token
    pub fn base_token(&self) -> &Token  {
        LiquidityPool::base_token(&self.liquidity_pool)
    }

    pub fn base_token_decimals(&self) -> TokenDecimalsType {
        LiquidityPool::base_token(&self.liquidity_pool).decimals()
    }

    pub fn base_token_amount(&self) -> AmountType {
        calc_token_amount(
            self.base_token_digits,
            LiquidityPool::base_token(&self.liquidity_pool).decimals(),
        )
    }

    pub fn min_base_token_balance(&self) -> AmountType {
        LiquidityPool::min_base_token_balance(&self.liquidity_pool)
    }

    pub fn is_low_base_token_balance(&self) -> bool {
        self.base_token_amount() <= self.min_base_token_balance()
    }

    pub fn available_base_token_amount(&self) -> AmountType {
        let result = self.base_token_amount() - self.min_base_token_balance();
        if result >= 0.0 {
            result
        } else {
            return AmountType::NAN
        }
    }

    // Quote token
    pub fn quote_token(&self) -> &Token  {
        LiquidityPool::quote_token(&self.liquidity_pool)
    }

    pub fn quote_token_amount(&self) -> AmountType {
        calc_token_amount(
            self.quote_token_digits,
            LiquidityPool::quote_token(&self.liquidity_pool).decimals(),
        )
    }

    pub fn quote_token_decimals(&self) -> TokenDecimalsType {
        LiquidityPool::quote_token(&self.liquidity_pool).decimals()
    }

    pub fn min_quote_token_balance(&self) -> AmountType {
        LiquidityPool::min_quote_token_balance(&self.liquidity_pool)
    }

    pub fn is_low_quote_token_balance(&self) -> bool {
        self.quote_token_amount() <= self.min_quote_token_balance()
    }

    pub fn available_quote_token_amount(&self) -> AmountType {
        let result = self.quote_token_amount() - self.min_quote_token_balance();
        if result >= 0.0 {
            result
        } else {
            return AmountType::NAN
        }
    }
    
    pub fn calc_quote_token_amount_to_buy(&self, base_token_amount: AmountType) -> AmountType {
        match &self.liquidity_pool{
            LiquidityPool::RaydiumAmm(_) => RaydiumAmmLp::calc_quote_token_amount_to_buy(self.available_base_token_amount(), self.available_quote_token_amount(), self.fee_ratio, base_token_amount),
            LiquidityPool::OrcaWhirlpool(_lp) => RaydiumAmmLp::calc_quote_token_amount_to_buy(self.available_base_token_amount(), self.available_quote_token_amount(), self.fee_ratio, base_token_amount),
        }
    }

    pub fn calc_price_to_buy(&self) -> AmountType{
        self.calc_quote_token_amount_to_buy(1.0)
    }

    pub fn calc_quote_token_amount_for_selling(&self, base_token_amount: AmountType) -> AmountType {
        match &self.liquidity_pool{
            LiquidityPool::RaydiumAmm(_) => RaydiumAmmLp::calc_quote_token_amount_for_selling(self.available_base_token_amount(), self.available_quote_token_amount(), self.fee_ratio, base_token_amount),
            LiquidityPool::OrcaWhirlpool(_) => OrcaWhirlpoolLp::calc_quote_token_amount_for_selling(self.available_base_token_amount(), self.available_quote_token_amount(), self.fee_ratio, base_token_amount),
        }
    }

    pub fn calc_price_for_selling(&self) -> AmountType {
        self.calc_quote_token_amount_for_selling(1.0)
    }
}

pub fn calc_token_amount(digits: TokenDigitsType, decimals: TokenDecimalsType) -> AmountType {
    // 10.pow() should not overflow because decimals is u8.
    digits as AmountType / 10u64.pow(decimals as u32) as AmountType
}

pub fn read_from_file(file_name: &str) -> String {
    let mut file = File::open(file_name).unwrap();

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();
    file_contents
}

#[cfg(test)]
mod tests {
    use {
        super::{TokenAccount, Wallet},
        crate::{
            calc_token_amount, read_from_file, test_util::tests::{
                create_liquidity_pool, create_lp_state, create_orca_whirlpool_lp, create_raydium_amm_lp, MOCK_wSOL_TOKEN, MOCK_wSOL_TOKEN_ACCOUNT, MOCK_USDC_TOKEN, MOCK_USDC_TOKEN_ACCOUNT, MOCK_USDT_TOKEN_ACCOUNT
            }, AccountType, AmountType, LiquidityGroupCode, LiquidityPool, LiquidityPoolState, OrcaWhirlpoolLp, RaydiumAmmLp, TokenCode, TokenDigitsType, ONE_BP_DECIMAL
        },
        solana_sdk::{pubkey::Pubkey, signature::Keypair},
        std::{collections::HashMap, fs, ops::Mul},
        tempfile::NamedTempFile,
    };

    #[test]
    fn test_read_from_file() {
        const TEST_CONTENTS: &str = "Test File Contents";

        let file = NamedTempFile::new().expect("Failed to create Orca config file");
        let file_path = file.path();
        fs::write(file_path, TEST_CONTENTS).expect("Failed to write to the temp file!");

        assert_eq!(TEST_CONTENTS, read_from_file(file_path.to_str().unwrap()));
    }

    #[test]
    fn test_account_type() {
        assert_eq!(
            AccountType::OrcaWhirlpoolAccount,
            AccountType::try_from(&format!("{:?}", AccountType::OrcaWhirlpoolAccount) as &str).unwrap(),
        );
        assert_eq!(
            AccountType::OrcaWhirlpoolAccount,
            AccountType::try_from(&format!("{:?}", AccountType::OrcaWhirlpoolAccount) as &str).unwrap(),
        );
        assert_eq!(
            AccountType::OrcaWhirlpoolProgramId,
            AccountType::try_from(&format!("{:?}", AccountType::OrcaWhirlpoolProgramId) as &str)
                .unwrap(),
        );
        assert_eq!(
            AccountType::RaydiumAmmPoolAccount,
            AccountType::try_from(&format!("{:?}", AccountType::RaydiumAmmPoolAccount) as &str).unwrap(),
        );
        assert_eq!(
            AccountType::RaydiumAmmPoolProgramId,
            AccountType::try_from(&format!("{:?}", AccountType::RaydiumAmmPoolProgramId) as &str)
                .unwrap(),
        );
        assert_eq!(
            AccountType::RaydiumAmmPoolVaultForBaseToken,
            AccountType::try_from(&format!("{:?}", AccountType::RaydiumAmmPoolVaultForBaseToken) as &str).unwrap(),
        );
        assert_eq!(
            AccountType::RaydiumAmmPoolVaultForQuoteToken,
            AccountType::try_from(
                &format!("{:?}", AccountType::RaydiumAmmPoolVaultForQuoteToken) as &str
            )
            .unwrap(),
        );
    }

    #[test]
    fn test_token_code() {
        assert_eq!(
            TokenCode::wSOL,
            TokenCode::try_from(TokenCode::wSOL.to_string().as_str()).unwrap()
        );
        assert_eq!(
            TokenCode::bSOL,
            TokenCode::try_from(TokenCode::bSOL.to_string().as_str()).unwrap()
        );
        assert_eq!(
            TokenCode::mSOL,
            TokenCode::try_from(TokenCode::mSOL.to_string().as_str()).unwrap()
        );
        assert_eq!(
            TokenCode::stSOL,
            TokenCode::try_from(TokenCode::stSOL.to_string().as_str()).unwrap()
        );
        assert_eq!(
            TokenCode::USDC,
            TokenCode::try_from(TokenCode::USDC.to_string().as_str()).unwrap()
        );
        assert_eq!(
            TokenCode::USDT,
            TokenCode::try_from(TokenCode::USDT.to_string().as_str()).unwrap()
        );
    }

    #[test]
    fn test_liquidity_group_code() {
        assert_eq!(
            LiquidityGroupCode::SOL_USD,
            LiquidityGroupCode::try_from(LiquidityGroupCode::SOL_USD.to_string().as_str()).unwrap()
        );
    }

    #[test]
    fn test_raydium_amm_liquidity_pool() {
        let address = Pubkey::new_unique();
        let program_id = Pubkey::new_unique();
        let name = "Test Raydium Amm";
        let lp = LiquidityPool::RaydiumAmm(RaydiumAmmLp {
            liquidity_group: LiquidityGroupCode::SOL_USD,
            program_id,
            name: name.to_string(),
            address,
            authority: Pubkey::new_unique(),
            base_token: MOCK_wSOL_TOKEN,
            base_token_vault: Pubkey::new_unique(),
            min_base_token_balance: 0.01,
            quote_token: MOCK_USDC_TOKEN,
            quote_token_vault: Pubkey::new_unique(),
            min_quote_token_balance: 1.0,
        });

        assert_eq!(LiquidityPool::pool_type(&lp), &AccountType::RaydiumAmmPoolAccount);
        assert_eq!(LiquidityPool::liquidity_group(&lp), LiquidityGroupCode::SOL_USD);
        assert_eq!(LiquidityPool::address(&lp), &address);
        assert_eq!(LiquidityPool::name(&lp), name);
        assert_eq!(LiquidityPool::accounts(&lp).len(), 3);
        assert_eq!(LiquidityPool::program_id(&lp), &program_id);
        assert_eq!(LiquidityPool::base_token(&lp).decimals(), MOCK_wSOL_TOKEN.decimals());
        assert_eq!(LiquidityPool::quote_token(&lp).decimals(), MOCK_USDC_TOKEN.decimals());
    }

    #[test]
    fn test_calc_raydium_amm_quote_amount_to_buy(){
        assert!((RaydiumAmmLp::calc_quote_token_amount_to_buy(100.0, 100_000.0, 0.01, 0.01) - 10.102020303049652).abs() < ONE_BP_DECIMAL);
        assert!((RaydiumAmmLp::calc_quote_token_amount_to_buy(100.0, 100_000.0, 0.01, 1.0) - 1020.304050607).abs() < ONE_BP_DECIMAL);
        assert!((RaydiumAmmLp::calc_quote_token_amount_to_buy(100.0, 100_000.0, 0.01, 99.99) - 1_009_999_999.9994832).abs() < ONE_BP_DECIMAL);

        // Not enough liquidity
        assert!(RaydiumAmmLp::calc_quote_token_amount_to_buy(100.0, 100_000.0, 0.01, 100.0).is_nan());
        assert!(RaydiumAmmLp::calc_quote_token_amount_to_buy(100.0, 100_000.0, 0.01, 100.01).is_nan());
    }

    #[test]
    fn test_calc_raydium_amm_quote_amount_for_selling(){
        assert!((RaydiumAmmLp::calc_quote_token_amount_for_selling(100.0, 100_000.0, 0.01, 101.0) - 49997.49987499375).abs() < ONE_BP_DECIMAL);
        assert!((RaydiumAmmLp::calc_quote_token_amount_for_selling(100.0, 100_000.0, 0.01, 1.0) - 980.295079).abs() < ONE_BP_DECIMAL);
        assert!((RaydiumAmmLp::calc_quote_token_amount_for_selling(100.0, 100_000.0, 0.01, 0.01) -  9.899019997028518).abs() < ONE_BP_DECIMAL);
    }

    #[test]
    fn test_orca_whirlpool_liquidity_pool() {
        let address = Pubkey::new_unique();
        let name = "Test Orca Whirlpool";
        let lp = LiquidityPool::OrcaWhirlpool(OrcaWhirlpoolLp {
            liquidity_group: LiquidityGroupCode::SOL_USD,
            name: name.to_string(),
            address,
            base_token: MOCK_wSOL_TOKEN,
            min_base_token_balance: 0.01,
            quote_token: MOCK_USDC_TOKEN,
            min_quote_token_balance: 1.0,
            tick: 2,
        });
        assert_eq!(LiquidityPool::pool_type(&lp), &AccountType::OrcaWhirlpoolAccount);
        assert_eq!(LiquidityPool::liquidity_group(&lp), LiquidityGroupCode::SOL_USD);
        assert_eq!(LiquidityPool::address(&lp), &address);
        assert_eq!(LiquidityPool::name(&lp), name);
        assert_eq!(LiquidityPool::accounts(&lp).len(), 1);
        assert_eq!(LiquidityPool::base_token(&lp).decimals, MOCK_wSOL_TOKEN.decimals());
        assert_eq!(LiquidityPool::quote_token(&lp).decimals(), MOCK_USDC_TOKEN.decimals());
    }

    #[test]
    fn test_create_lp_state() {
        let lp = create_liquidity_pool(AccountType::RaydiumAmmPoolAccount);
        let lp_state = LiquidityPoolState::new(lp.clone());
        assert_eq!(lp, lp_state.liquidity_pool);
    }

    #[test]
    fn test_is_lp_state_computable() {
        let lp = create_raydium_amm_lp();
        let lps = create_lp_state(LiquidityPool::RaydiumAmm(lp.clone()));
        assert!(lps.is_computable());

        let init_quote_token_digits = lps.quote_token_digits;

        // Amounts are not defined
        let mut lps = create_lp_state(LiquidityPool::RaydiumAmm(lp.clone()));
        lps.healthy = true;
        assert!(lps.is_computable());
        lps.quote_token_digits = 0;
        assert!(!lps.is_computable());
        lps.quote_token_digits = init_quote_token_digits;
        assert!(lps.is_computable());

        // Undefined fee        
        lps.fee_ratio = f64::NAN;
        assert!(!lps.is_computable());

        // No updates after the state was loaded from snapshot.
        lps.fee_ratio = 0.01;
        assert!(lps.is_computable());
        lps.latest_upd_txn_sig = None;
        assert!(!lps.is_computable());
    }

    #[test]
    fn test_token_amount_calculation() {
        assert!(calc_token_amount(123456789, 0) - 123456789.0 < ONE_BP_DECIMAL);
        assert!(calc_token_amount(123456789, 1) - 12345678.9 < ONE_BP_DECIMAL);
        assert!(calc_token_amount(123456789, 2) - 1234567.89 < ONE_BP_DECIMAL);
        assert!(calc_token_amount(123456789, 3) - 123456.789 < ONE_BP_DECIMAL);
        assert!(calc_token_amount(123456789, 4) - 12345.6789 < ONE_BP_DECIMAL);
        assert!(calc_token_amount(123456789, 5) - 1234.56789 < ONE_BP_DECIMAL);
        assert!(calc_token_amount(123456789, 6) - 123.456789 < ONE_BP_DECIMAL);
        assert!(calc_token_amount(123456789, 7) - 12.3456789 < ONE_BP_DECIMAL);
        assert!(calc_token_amount(123456789, 8) - 1.23456789 < ONE_BP_DECIMAL);
        assert!(calc_token_amount(123456789, 9) - 0.123456789 < ONE_BP_DECIMAL);

        let lp = create_orca_whirlpool_lp();
        let mut lps = LiquidityPoolState::new(LiquidityPool::OrcaWhirlpool(lp));
        lps.base_token_digits = 1000;
        lps.quote_token_digits = 2000;
        assert!(lps.base_token_amount() - 10.0 < ONE_BP_DECIMAL);
        assert!(lps.quote_token_amount() - 0.2 < ONE_BP_DECIMAL);
    }

    #[test]
    fn test_wallet() {
        let keypair1: Keypair = Keypair::new();

        let mut token_accounts:HashMap<TokenCode, TokenAccount> = HashMap::new();
        token_accounts.insert(MOCK_wSOL_TOKEN_ACCOUNT.code(), MOCK_wSOL_TOKEN_ACCOUNT.clone());
        token_accounts.insert(MOCK_USDC_TOKEN_ACCOUNT.code(), MOCK_USDC_TOKEN_ACCOUNT.clone());
        token_accounts.insert(MOCK_USDT_TOKEN_ACCOUNT.code(), MOCK_USDT_TOKEN_ACCOUNT.clone());


        let wallet = Wallet{ keypair: keypair1.insecure_clone(), token_accounts };
        assert_eq!(wallet.keypair(), &keypair1);
        assert_eq!(wallet.token_accounts.len(), 3);
        assert_eq!(
            wallet.token_account(&TokenCode::wSOL).unwrap().address(),
            MOCK_wSOL_TOKEN_ACCOUNT.address()
        );
        assert_eq!(
            wallet.token_account(&TokenCode::USDC).unwrap().min_balance(),
            MOCK_USDC_TOKEN_ACCOUNT.min_balance()
        );
    }

    #[test]
    fn test_token_account() {
        let pubkey: Pubkey = Pubkey::new_unique();
        const MIN_AMOUNT: AmountType = 1.23456;
        let token_account = TokenAccount {
            code: TokenCode::wSOL,
            address: pubkey,
            min_balance: MIN_AMOUNT,
        };
        assert_eq!(token_account.code(), TokenCode::wSOL);
        assert_eq!(token_account.address(), &pubkey);
        assert_eq!(token_account.min_balance(), MIN_AMOUNT);
    }

    #[test]
    fn test_min_token_balance() {
        const AMOUNT1: AmountType = 1.23;
        const AMOUNT2: AmountType = 2.34;
        const AMOUNT3: AmountType = 3.45;
        const AMOUNT4: AmountType = 4.56;

        let mut orca_lp = create_orca_whirlpool_lp();
        orca_lp.min_base_token_balance = AMOUNT1;
        orca_lp.min_quote_token_balance = AMOUNT2;
        let lp = LiquidityPool::OrcaWhirlpool(orca_lp);
        assert_eq!(LiquidityPool::min_base_token_balance(&lp), AMOUNT1);
        assert_eq!(LiquidityPool::min_quote_token_balance(&lp), AMOUNT2);
        let lp_state = create_lp_state(lp);
        assert_eq!(lp_state.min_base_token_balance(), AMOUNT1);
        assert_eq!(lp_state.min_quote_token_balance(), AMOUNT2);

        let mut raydium_lp = create_raydium_amm_lp();
        raydium_lp.min_base_token_balance = AMOUNT3;
        raydium_lp.min_quote_token_balance = AMOUNT4;
        let lp = LiquidityPool::RaydiumAmm(raydium_lp.clone());
        assert_eq!(LiquidityPool::min_base_token_balance(&lp), AMOUNT3);
        assert_eq!(LiquidityPool::min_quote_token_balance(&lp), AMOUNT4);
        let lp_state = create_lp_state(lp);
        assert_eq!(lp_state.min_base_token_balance(), AMOUNT3);
        assert_eq!(lp_state.min_quote_token_balance(), AMOUNT4);
    }

    #[test]
    fn test_lp_state_low_token_balance() {
        // Good balance
        let lp = create_raydium_amm_lp();
        let mut lp_state = create_lp_state(LiquidityPool::RaydiumAmm(lp.clone()));

        lp_state.base_token_digits = (lp.min_base_token_balance
            * 10u64.pow(lp.base_token.decimals as u32) as AmountType
            + 1 as AmountType) as TokenDigitsType;
        lp_state.quote_token_digits = (lp.min_quote_token_balance
            * 10u64.pow(lp.quote_token.decimals as u32) as AmountType
            + 1 as AmountType) as TokenDigitsType;
        assert!(!lp_state.is_low_base_token_balance());
        assert!(!lp_state.is_low_quote_token_balance());

        // Low balance
        lp_state.base_token_digits = (lp.min_base_token_balance
            * 10u64.pow(lp.base_token.decimals as u32) as AmountType) as TokenDigitsType;
        lp_state.quote_token_digits = (lp.min_quote_token_balance
            * 10u64.pow(lp.quote_token.decimals as u32) as AmountType) as TokenDigitsType;
        assert!(lp_state.is_low_base_token_balance());
        assert!(lp_state.is_low_quote_token_balance());
    }

    #[test]
    fn test_lp_state_available_token_amount(){
        let mut lp = create_raydium_amm_lp();
        lp.min_base_token_balance = 10.0;
        lp.min_quote_token_balance = 100.0;

        // Available
        let mut lp_state = create_lp_state(LiquidityPool::RaydiumAmm(lp.clone()));
        lp_state.base_token_digits = 11u64.mul(10u32.pow(lp.base_token.decimals() as u32) as TokenDigitsType);
        assert_eq!(lp_state.available_base_token_amount(),  1.0);

        lp_state.quote_token_digits = 110u64.mul(10u32.pow(lp.quote_token.decimals() as u32) as TokenDigitsType);
        assert_eq!(lp_state.available_quote_token_amount(),  10.0);

        // Unavailable
        lp_state.base_token_digits = 9u64.mul(10u32.pow(lp.base_token.decimals() as u32) as TokenDigitsType);
        assert!(!lp_state.available_base_token_amount().is_finite());
        lp_state.quote_token_digits = 99u64.mul(10u32.pow(lp.quote_token.decimals() as u32) as TokenDigitsType);
        assert!(!lp_state.available_quote_token_amount().is_finite());
    }
}
