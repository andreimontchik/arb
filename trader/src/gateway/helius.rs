use {
    super::Gateway,
    crate::ComputedUnitsPriceType,
    anyhow::{bail, Result},
    common::CommonError,
    helius::{
        types::{Cluster, GetPriorityFeeEstimateOptions, GetPriorityFeeEstimateRequest},
        Helius, HeliusFactory,
    },
    serde::Deserialize,
    serde_json::Value,
    solana_client::{
        rpc_config::RpcSendTransactionConfig,
        rpc_response::{Response, RpcSimulateTransactionResult},
    },
    solana_sdk::{hash::Hash, pubkey::Pubkey, signature::Signature, transaction::Transaction},
    tokio::{self, runtime::Runtime},
};

const GET_PRIORITY_FEE_REQUEST_OPTIONS: GetPriorityFeeEstimateOptions = GetPriorityFeeEstimateOptions {
    priority_level: None,
    include_all_priority_fee_levels: None,
    transaction_encoding: None,
    lookback_slots: None,
    recommended: Some(true),
    include_vote: None,
};

fn to_cluster(cluster_name: &str) -> Result<Cluster> {
    match cluster_name {
        "devnet" => Ok(Cluster::Devnet),
        "mainnet" => Ok(Cluster::MainnetBeta),
        _ => bail!("Invalid Cluster name ({})", cluster_name),
    }
}

#[derive(Deserialize)]
pub struct HeliusConfig {
    api_key: String,
    cluster: String,
    txn_skip_preflight: bool,
    txn_max_retries: usize,
}

impl HeliusConfig {
    pub fn new(config: Value) -> Self {
        serde_json::from_value(config).unwrap()
    }
}

pub struct HeliusGateway {
    tokio_runtime: Runtime,
    helius_client: Helius,
    txn_config: RpcSendTransactionConfig,
}

impl Gateway for HeliusGateway {
    fn new(config: Value) -> Self {
        let config = HeliusConfig::new(config);

        let tokio_runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let factory = HeliusFactory::new(&config.api_key);
        let cluster = to_cluster(&config.cluster).unwrap();
        let helius_client = factory.create(cluster).unwrap();

        let mut txn_config = RpcSendTransactionConfig::default();
        txn_config.skip_preflight = config.txn_skip_preflight;
        txn_config.max_retries = Some(config.txn_max_retries);

        HeliusGateway {
            tokio_runtime,
            helius_client,
            txn_config,
        }
    }

    fn get_latest_blockhash(&self) -> Result<Hash> {
        Ok(self.helius_client.connection().get_latest_blockhash()?)
    }

    fn get_recent_cu_price(&self, account: &Pubkey) -> Result<ComputedUnitsPriceType> {
        let request = GetPriorityFeeEstimateRequest {
            transaction: None,
            account_keys: Some(vec![account.to_string()]),
            options: Some(GET_PRIORITY_FEE_REQUEST_OPTIONS),
        };

        let response = self
            .tokio_runtime
            .block_on(self.helius_client.rpc().get_priority_fee_estimate(request))?;

        Ok(response
            .priority_fee_estimate
            .ok_or(CommonError::CalculationError {
                error: "The priority fee was not returned!".to_string(),
            })? as ComputedUnitsPriceType)
    }

    fn send_transaction(&self, txn: &Transaction) -> Result<Signature> {
        Ok(
            /*
                        self
                        .helius_client
                        .connection()
                        .send_and_confirm_transaction_with_spinner_and_config(
                            txn,
                            CommitmentConfig::confirmed(),
                            self.txn_config,
                        )?
            */
            self.helius_client
                .connection()
                .send_transaction_with_config(txn, self.txn_config)?,
        )
    }

    fn simulate_transaction(&self, txn: &Transaction) -> Result<Response<RpcSimulateTransactionResult>> {
        Ok(self.helius_client.connection().simulate_transaction(txn)?)
    }
}

#[cfg(test)]
mod tests {
    use {super::to_cluster, helius::types::Cluster};

    #[test]
    fn test_to_cluster() {
        assert_eq!(to_cluster(&"mainnet").unwrap(), Cluster::MainnetBeta);
        assert_eq!(to_cluster(&"devnet").unwrap(), Cluster::Devnet);
        assert!(to_cluster(&"dummy").is_err());
    }
}
