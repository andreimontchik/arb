mod arbitrage_cache;
mod arbitrage_persister;
mod arbitrage_publisher;

mod orca_util;
mod raydium_util;

use {
    anyhow::Result,
    common::{message::ArbitrageMessage, LiquidityGroupCode},
    serde_json::Value,
    solana_sdk::pubkey::Pubkey,
    std::error,
    thiserror::Error,
};
pub use {
    arbitrage_cache::ArbitrageCache, arbitrage_persister::ArbitragePersister,
    arbitrage_publisher::ArbitragePublisher,
};

#[derive(Error, Debug)]
pub enum ArbitrageError {
    #[error("({})",lg.to_string())]
    UnsupportedLiquidityGroup { lg: LiquidityGroupCode },
    #[error("({address})")]
    UnrecognizedLiquidityPool { address: Pubkey },
    #[error("({address})")]
    UnrecognizedRaydiumVoteAccount { address: Pubkey },
    #[error("({msg})")]
    UnrecognizedRaydiumAmmAccountUpdateStatus { msg: String },
    #[error("({code})")]
    UnsupportedTokenCode { code: String },
    #[error("({msg})")]
    CalculationError { msg: String },
    #[error("({msg})")]
    TokenAmountsCalculationFailure { msg: String },
    #[error("({msg}")]
    PriceCalculationFailure { msg: String },
    #[error("({msg})")]
    FeeCalculationFailure { msg: String },
    #[error("({msg})")]
    PeristingError { msg: String },
    #[error("({0})")]
    CustomError(Box<dyn error::Error + Send + Sync>),
}

pub trait ArbitrageExecutor {
    fn new(config: Value) -> Self;
    fn execute(&mut self, arbitrage: &ArbitrageMessage) -> Result<()>;
}
