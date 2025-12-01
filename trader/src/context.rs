use {
    crate::{
        gateway::Gateway, ComputedUnitsLimitType, ComputedUnitsPriceType, ContextConfig,
        TimeIntervalType, TraderMetricKey,
    },
    anyhow::Result,
    common::{
        config::{to_pubkey, to_raydium_amm, to_token, RaydiumAmmConfig, TokensConfig, WalletConfig},
        metrics::MetricsCollector,
        CommonError, LiquidityPool, Token, TokenCode, Wallet,
    },
    log::{error, info},
    serde_json::Value,
    solana_sdk::{hash::Hash, pubkey::Pubkey},
    std::{
        collections::HashMap,
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc, Mutex,
        },
        thread,
        time::{self, Instant},
    },
};

pub struct Context {
    pub(crate) spl_token_program_id: Pubkey,
    pub(crate) wallet: Wallet,
    pub(crate) tokens: HashMap<TokenCode, Token>,
    pub(crate) liquidity_pools: HashMap<Pubkey, LiquidityPool>,
    pub(crate) latest_blockhash: Option<Hash>,
    pub(crate) arb_cu_limit: ComputedUnitsLimitType,
    pub(crate) swap_cu_limit: ComputedUnitsLimitType,
    pub(crate) min_cu_price: ComputedUnitsPriceType,
    pub(crate) max_cu_price: ComputedUnitsPriceType,
    pub(crate) recent_cu_prices: HashMap<Pubkey, ComputedUnitsPriceType>,
}

impl Context {
    pub fn new(
        context_config: &ContextConfig,
        wallet_config: &WalletConfig,
        tokens_config: &TokensConfig,
        raydium_config: &RaydiumAmmConfig,
    ) -> Result<Self> {
        info!("Registering Wallet from ({:?})", &wallet_config);
        let wallet = Wallet::new(wallet_config)?;

        let mut result = Context {
            spl_token_program_id: to_pubkey(&context_config.spl_token_program_id)?,
            wallet,
            tokens: HashMap::new(),
            liquidity_pools: HashMap::new(),
            latest_blockhash: None,
            arb_cu_limit: context_config.arb_cu_limit,
            swap_cu_limit: context_config.swap_cu_limit,
            min_cu_price: context_config.min_cu_price,
            max_cu_price: context_config.max_cu_price,
            recent_cu_prices: HashMap::new(),
        };

        info!("Registering tokens from ({:?})", &tokens_config);
        for config in tokens_config.tokens() {
            let token = to_token(&config)?;
            info!("Adding the Token ({:?})", token);
            result.tokens.insert(token.code(), token);
        }

        for lp_config in &raydium_config.amm_pools {
            if lp_config.enabled {
                info!("Adding the Raydium AMM Liquidity Pool ({:?})", lp_config);
                let lp = to_raydium_amm(
                    &raydium_config.amm_program_id,
                    &raydium_config.authority,
                    raydium_config.min_base_token_balance,
                    raydium_config.min_quote_token_balance,
                    &lp_config,
                    &result.tokens,
                )
                .unwrap();
                result.liquidity_pools.insert(*LiquidityPool::address(&lp), lp);
            } else {
                info!(
                    "Ignoring the disabled Raydium AMM Liquidity Pool ({:?})",
                    lp_config
                );
            }
        }

        Ok(result)
    }

    pub fn liquidity_pool(&self, address: &Pubkey) -> Result<&LiquidityPool> {
        self.liquidity_pools
            .get(address)
            .ok_or(CommonError::InvalidLiquidityPool { address: *address }.into())
    }

    pub fn liquidity_pools(&self) -> &HashMap<Pubkey, LiquidityPool> {
        &self.liquidity_pools
    }

    pub fn wallet(&self) -> &Wallet {
        &self.wallet
    }

    pub fn latest_blockhash(&self) -> Result<Hash> {
        self.latest_blockhash.ok_or(
            CommonError::InvalidState {
                error: "Undefined latest blockhash!".to_string(),
            }
            .into(),
        )
    }

    pub fn update_latest_blockhash(&mut self, block_hash: Hash) {
        info!("Updating the latest blockhash to ({}).", block_hash);
        self.latest_blockhash = Some(block_hash);
    }

    pub fn recent_cu_price(&self, account_id: &Pubkey) -> Option<ComputedUnitsPriceType> {
        self.recent_cu_prices.get(account_id).copied()
    }

    pub fn update_recent_cu_price(&mut self, account_id: &Pubkey, cu_price: ComputedUnitsPriceType) {
        info!(
            "Updating recent CU price for the program ({}) to ({}).",
            account_id, cu_price
        );
        self.recent_cu_prices.insert(*account_id, cu_price);
    }

    #[inline]
    pub fn arb_cu_limit(&self) -> ComputedUnitsLimitType {
        self.arb_cu_limit
    }

    #[inline]
    pub fn swap_cu_limit(&self) -> ComputedUnitsLimitType {
        self.swap_cu_limit
    }
}

pub struct ContextUpdater {
    context: Arc<Mutex<Context>>,
    update_period_sec: TimeIntervalType,
    updater_thread_handle: Option<thread::JoinHandle<()>>,
    should_run: Arc<AtomicBool>,
}

impl ContextUpdater {
    pub fn new(update_period_sec: TimeIntervalType, context: Arc<Mutex<Context>>) -> Self {
        ContextUpdater {
            context,
            update_period_sec,
            updater_thread_handle: None,
            should_run: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn start<T: Gateway, M: MetricsCollector>(
        &mut self,
        gateway_config: Value,
        metrics_collector_config: Value,
    ) {
        info!("Starting the ContextUpdater...");

        let should_run = Arc::clone(&self.should_run);
        let update_period_sec = self.update_period_sec.clone();
        let gateway_config = gateway_config.clone();

        let spl_token_prigram_id = self.context.lock().unwrap().spl_token_program_id;
        let context = self.context.clone();

        // Spin off the context update thread
        let thread_handle = std::thread::spawn(move || {
            let gateway = T::new(gateway_config);
            let metrics_collector = M::new(metrics_collector_config);

            while should_run.load(Ordering::Relaxed) {
                // Latest blockhash
                let start = Instant::now();
                match gateway.get_latest_blockhash() {
                    Ok(hash) => {
                        context.lock().unwrap().update_latest_blockhash(hash);
                    }
                    Err(error) => error!("Failed to get the latest blockhash! {:?}", error),
                }
                metrics_collector
                    .duration(&TraderMetricKey::GetLatestBlockHashDuration, start.elapsed());

                // Recent CU prices
                let start = Instant::now();
                match gateway.get_recent_cu_price(&spl_token_prigram_id) {
                    Ok(cu_price) => context
                        .lock()
                        .unwrap()
                        .update_recent_cu_price(&spl_token_prigram_id, cu_price),
                    Err(error) => error!(
                        "Failed to get the recent CU price for the program account ({})! ({:?})",
                        &spl_token_prigram_id, error
                    ),
                }
                metrics_collector.duration(&TraderMetricKey::GetRecentCuPriceDuration, start.elapsed());

                thread::sleep(time::Duration::from_secs(update_period_sec));
            }
        });

        // Give some time to the context update thread to take off.
        thread::sleep(time::Duration::from_secs(1));
        if thread_handle.is_finished() {
            error!("The RPC client thread finished unexpectedy!");
            if let Err(panic) = thread_handle.join() {
                panic!("The RPC client thread panicked! {:?}", panic);
            }
        } else {
            self.updater_thread_handle = Some(thread_handle);
        }

        info!("The ContextUpdater started.");
    }

    pub fn stop(&mut self) {
        info!("Stopping the ContextUpdater...");
        self.should_run.store(false, Ordering::Relaxed);
        if let Some(handle) = self.updater_thread_handle.take() {
            if let Err(err) = handle.join() {
                error!("Error joining the ContextUpdate thread handle {:?}", err);
            }
        }
        info!("ContextUpdater stopped.");
    }
}

#[cfg(test)]
mod tests {

    use {
        crate::{
            test_util::tests::{generate_context, MockGateway, MOCK_CU_PRICE1},
            ContextUpdater,
        },
        common::metrics::NoopMetricsCollector,
        serde_json::Value,
        solana_sdk::{hash::Hash, pubkey::Pubkey},
        std::{
            sync::{Arc, Mutex},
            thread, time,
        },
    };

    #[test]
    fn test_latest_blockhash() {
        let mut context = generate_context();

        context.latest_blockhash = None;
        assert!(context.latest_blockhash().is_err());

        let hash = Hash::new_unique();
        context.update_latest_blockhash(hash);
        assert_eq!(context.latest_blockhash().unwrap(), hash);
    }

    #[test]
    fn test_recent_cu_prices() {
        let mut context = generate_context();
        let spl_token_program_id = context.spl_token_program_id;
        assert_eq!(context.recent_cu_prices.len(), 0);
        assert_eq!(context.liquidity_pools.len(), 2);

        context.update_recent_cu_price(&spl_token_program_id, MOCK_CU_PRICE1);

        assert_eq!(context.recent_cu_prices.len(), 1);
        assert_eq!(
            context.recent_cu_price(&spl_token_program_id).unwrap(),
            MOCK_CU_PRICE1
        );
        assert!(context.recent_cu_price(&Pubkey::new_unique()).is_none());
    }

    #[test]
    fn test_context_updater() {
        let context = generate_context();
        let context = Arc::new(Mutex::new(context));

        let mut context_updater = ContextUpdater::new(5, context.clone());
        assert!(context_updater.updater_thread_handle.is_none());

        let gateway_config_str = "{\"gateway\": \"Dummy Gateway configuration\"}";
        let gateway_config: Value = serde_json::from_str(&gateway_config_str).unwrap();

        let metrics_collector_config_str = "{\"metrics\": \"Dummy Metrics Collector configuration\"}";
        let metrics_collector_config = serde_json::from_str(&metrics_collector_config_str).unwrap();

        context_updater
            .start::<MockGateway, NoopMetricsCollector>(gateway_config, metrics_collector_config);
        while context_updater.updater_thread_handle.is_none() {
            thread::sleep(time::Duration::from_micros(100));
        }
        assert!(!context_updater
            .updater_thread_handle
            .as_ref()
            .unwrap()
            .is_finished());

        context_updater.stop();
    }
}
