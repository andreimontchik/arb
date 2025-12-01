mod helius;

pub use helius::HeliusGateway;
use {
    crate::ComputedUnitsPriceType,
    anyhow::Result,
    serde_json::Value,
    solana_client::rpc_response::{Response, RpcSimulateTransactionResult},
    solana_sdk::{hash::Hash, pubkey::Pubkey, signature::Signature, transaction::Transaction},
};

pub trait Gateway {
    fn new(config: Value) -> Self;
    fn get_latest_blockhash(&self) -> Result<Hash>;
    fn get_recent_cu_price(&self, account: &Pubkey) -> Result<ComputedUnitsPriceType>;
    fn send_transaction(&self, txn: &Transaction) -> Result<Signature>;
    fn simulate_transaction(&self, txn: &Transaction) -> Result<Response<RpcSimulateTransactionResult>>;
}
