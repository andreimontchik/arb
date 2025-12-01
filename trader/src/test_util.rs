pub mod tests {
    pub use common::test_util::tests::{
        create_liquidity_pool, create_raydium_amm_lp, create_raydium_amm_pool_config,
    };
    use {
        crate::{gateway::Gateway, ComputedUnitsLimitType, ComputedUnitsPriceType, Context},
        anyhow::Result,
        common::{
            test_util::tests::{create_mock_tokens_hashmap, create_mock_wallet},
            LiquidityPool,
        },
        serde_json::Value,
        solana_client::rpc_response::{Response, RpcResponseContext, RpcSimulateTransactionResult},
        solana_sdk::{hash::Hash, pubkey::Pubkey, signature::Signature, transaction::Transaction},
        std::collections::HashMap,
    };

    pub const MOCK_ARB_CU_LIMIT: ComputedUnitsLimitType = 100_000;
    pub const MOCK_MIN_CU_PRICE: ComputedUnitsPriceType = 500_000;
    pub const MOCK_MAX_CU_PRICE: ComputedUnitsPriceType = 1_000_000;
    pub const MOCK_SWAP_CU_LIMIT: ComputedUnitsLimitType = 50_000;
    pub const MOCK_CU_PRICE1: ComputedUnitsPriceType = 111111;
    pub const MOCK_CU_PRICE2: ComputedUnitsPriceType = 222222;
    pub const MOCK_CU_COUNT: ComputedUnitsPriceType = 10_000;

    pub fn generate_context() -> Context {
        let mut result = Context {
            spl_token_program_id: Pubkey::new_unique(),
            wallet: create_mock_wallet(),
            tokens: create_mock_tokens_hashmap(),
            liquidity_pools: HashMap::new(),
            latest_blockhash: Some(Hash::new_unique()),
            arb_cu_limit: MOCK_ARB_CU_LIMIT,
            min_cu_price: MOCK_MIN_CU_PRICE,
            max_cu_price: MOCK_MAX_CU_PRICE,
            swap_cu_limit: MOCK_SWAP_CU_LIMIT,
            recent_cu_prices: HashMap::new(),
        };

        let mut lp = create_raydium_amm_lp();
        result
            .liquidity_pools
            .insert(lp.address, LiquidityPool::RaydiumAmm(lp));

        lp = create_raydium_amm_lp();
        result
            .liquidity_pools
            .insert(lp.address, LiquidityPool::RaydiumAmm(lp));

        result
    }

    pub struct MockGateway {}

    impl Gateway for MockGateway {
        fn new(_config: Value) -> Self {
            MockGateway {}
        }

        fn get_latest_blockhash(&self) -> Result<Hash> {
            Ok(Hash::new_unique())
        }

        fn get_recent_cu_price(&self, _account: &Pubkey) -> Result<ComputedUnitsPriceType> {
            Ok(MOCK_CU_PRICE1)
        }

        fn send_transaction(&self, _txn: &Transaction) -> Result<Signature> {
            Ok(Signature::new_unique())
        }

        fn simulate_transaction(
            &self,
            _txn: &Transaction,
        ) -> Result<Response<RpcSimulateTransactionResult>> {
            Ok(Response {
                context: RpcResponseContext {
                    slot: 1,
                    api_version: None,
                },
                value: RpcSimulateTransactionResult {
                    err: None,
                    logs: None,
                    accounts: None,
                    units_consumed: None,
                    return_data: None,
                    inner_instructions: None,
                },
            })
        }
    }
}
