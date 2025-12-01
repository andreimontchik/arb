use {
    crate::{
        arbitrage::ArbitragePublisher,
        config::PluginConfig,
        processor::{ArbitrageController, ProcessorManager},
    },
    anyhow::{bail, Result},
    common::{
        config::{
            to_orca_whirlpool, to_raydium_amm, to_token, OrcaConfig, RaydiumAmmConfig,
            TokenAccountsConfig, TokensConfig,
        },
        message::{AccountUpdateMessage, BlockUpdateMessage, Message, MessageFilter},
        metrics::statsd_metrics_collector::StatsdMetricsCollector,
        LiquidityPool, RaydiumAmmLp, Token, TokenAccount, TokenCode,
    },
    log::{debug, info},
    solana_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPlugin, GeyserPluginError, ReplicaAccountInfoV3, ReplicaAccountInfoVersions,
        ReplicaBlockInfoV3, ReplicaBlockInfoVersions, Result as GeyserPluginResult,
    },
    solana_sdk::{pubkey::Pubkey, slot_history::Slot},
    std::{
        collections::HashMap,
        sync::{
            atomic::{AtomicBool, Ordering},
            mpsc::{self, Sender},
        },
    },
};

#[derive(Debug)]
pub struct Plugin {
    sender: Sender<Message>,
    processor_manager: ProcessorManager,
    message_filter: MessageFilter,
    tokens: HashMap<TokenCode, Token>,
    startup_completed: AtomicBool,
}

impl Plugin {
    fn new() -> Plugin {
        let (sender, receiver) = mpsc::channel::<Message>();
        Plugin {
            sender,
            processor_manager: ProcessorManager::new(receiver),
            message_filter: MessageFilter::new(),
            tokens: HashMap::new(),
            startup_completed: AtomicBool::new(false),
        }
    }

    fn register_tokens(&mut self, tokens_config: TokensConfig) -> Result<()> {
        for token_config in tokens_config.tokens() {
            let token = to_token(&token_config)?;
            self.tokens.insert(token.code(), token);
        }
        Ok(())
    }

    fn register_token_accounts(&mut self, token_accounts_config: TokenAccountsConfig) -> Result<()> {
        for config in token_accounts_config.token_accounts() {
            let token_account = TokenAccount::try_from(config)?;
            self.message_filter.add_account(&token_account.address());
            self.send_message(Message::TokenAccountConfiguration(token_account))?;
        }
        Ok(())
    }

    fn register_orca(&mut self, orca_config: OrcaConfig) -> Result<()> {
        for wp_config in &orca_config.whirlpools {
            if wp_config.enabled {
                info!("Registeding Orca Whirlpool ({:?})", wp_config);

                let lp = to_orca_whirlpool(wp_config, &self.tokens)?;
                self.message_filter.add_account(LiquidityPool::address(&lp));
                self.send_message(Message::LiquidityPoolConfiguration(lp))?;
            } else {
                info!("Ignoring the disabled Orca Whirlpool ({:?})", wp_config);
            }
        }

        Ok(())
    }

    fn register_raydium(&mut self, raydium_config: RaydiumAmmConfig) -> Result<()> {
        for amm_config in &raydium_config.amm_pools {
            if amm_config.enabled {
                info!("Registeding Raydium AMM: ({:?})", amm_config);

                let lp = to_raydium_amm(
                    &raydium_config.amm_program_id,
                    &raydium_config.authority,
                    raydium_config.min_base_token_balance,
                    raydium_config.min_quote_token_balance,
                    amm_config,
                    &self.tokens,
                )?;
                if let LiquidityPool::RaydiumAmm(RaydiumAmmLp {
                    base_token_vault,
                    quote_token_vault,
                    ..
                }) = lp
                {
                    self.message_filter.add_account(LiquidityPool::address(&lp));
                    self.message_filter.add_account(&base_token_vault);
                    self.message_filter.add_account(&quote_token_vault);
                    self.send_message(Message::LiquidityPoolConfiguration(lp))?;
                } else {
                    bail!("Invalid configuration! Expected Raydium AMM config!".to_string());
                }
            } else {
                info!("Ignoring the disabled Raydium AMM: ({:?})", amm_config);
            }
        }

        Ok(())
    }

    fn handle_block_update(&self, msg: &ReplicaBlockInfoV3) -> Result<()> {
        if self.startup_completed.load(Ordering::Relaxed) {
            debug!("Handling block update. {:?}", msg);
            self.send_message(Message::BlockUpdate(BlockUpdateMessage {
                slot: msg.slot,
                block_time: msg.block_time,
                block_height: msg.block_height,
            }))?;
        };
        Ok(())
    }

    fn handle_account_update(&self, slot: Slot, msg: &ReplicaAccountInfoV3) -> Result<()> {
        if !self.message_filter.is_registered(msg.owner, msg.pubkey) {
            return Ok(());
        }

        debug!(
            "Handling account update. Slot: {}, Address: {:?}",
            slot,
            Pubkey::try_from(msg.pubkey)
        );

        let address = Pubkey::try_from(msg.pubkey)?;

        self.send_message(Message::AccountUpdate(AccountUpdateMessage {
            slot,
            address: address.clone(),
            // TODO: consider using the byte arrays pool instead of allocating new vec for msg.data on every message
            data: msg.data.to_vec(),
            txn_signature: msg.txn.map(|txn| *txn.signature()),
        }))
    }

    fn send_message(&self, msg: Message) -> Result<()> {
        Ok(self.sender.send(msg)?)
    }
}

impl GeyserPlugin for Plugin {
    fn name(&self) -> &'static str {
        "AsyncPlugin"
    }

    fn account_data_notifications_enabled(&self) -> bool {
        true
    }

    fn transaction_notifications_enabled(&self) -> bool {
        false
    }

    fn entry_notifications_enabled(&self) -> bool {
        false
    }

    fn on_load(&mut self, config_file: &str, _is_reload: bool) -> GeyserPluginResult<()> {
        solana_logger::setup_with_default("info");

        let config = PluginConfig::load(config_file);
        let tokens_config = TokensConfig::load(&config.tokens);
        let token_accounts_config = TokenAccountsConfig::load(&config.token_accounts);

        let orca_config_opt = if !config.orca.is_empty() {
            Some(OrcaConfig::load(&config.orca))
        } else {
            info!("Orca is is not configured.");
            None
        };

        let raydium_config_opt = if !config.raydium.is_empty() {
            Some(RaydiumAmmConfig::load(&config.raydium))
        } else {
            info!("Raydium is not configured.");
            None
        };

        //self.processor_manager.start::<MessagePersister<CsvSerializer>>(config.processor);
        self.processor_manager
            .start::<ArbitrageController<StatsdMetricsCollector, ArbitragePublisher>>(config.processor);

        // Register Tokens and Token accounts
        self.register_tokens(tokens_config)
            .map_err(|error| GeyserPluginError::Custom(error.into()))?;

        // Register Token Accounts
        self.register_token_accounts(token_accounts_config)
            .map_err(|error| GeyserPluginError::Custom(error.into()))?;

        // Register Orca accounts
        if let Some(orca_config) = orca_config_opt {
            self.register_orca(orca_config)
                .map_err(|error| GeyserPluginError::Custom(error.into()))?;
        }

        // Register Raydium Accounts
        if let Some(raydium_config) = raydium_config_opt {
            self.register_raydium(raydium_config)
                .map_err(|error| GeyserPluginError::Custom(error.into()))?;
        }

        Ok(())
    }

    fn on_unload(&mut self) {
        self.processor_manager.stop();
    }

    fn notify_end_of_startup(&self) -> GeyserPluginResult<()> {
        info!("The notify_end_of_startup is called.");
        self.startup_completed.store(true, Ordering::Relaxed);
        Ok(())
    }

    fn notify_block_metadata(&self, blockinfo: ReplicaBlockInfoVersions) -> GeyserPluginResult<()> {
        match blockinfo {
            ReplicaBlockInfoVersions::V0_0_1(_) => Ok(()), // Ignore
            ReplicaBlockInfoVersions::V0_0_2(_) => Ok(()), // Ignore
            ReplicaBlockInfoVersions::V0_0_3(block_info) => Ok(self
                .handle_block_update(block_info)
                .map_err(|error| GeyserPluginError::Custom(error.into()))?),
        }
    }

    fn update_account(
        &self,
        msg_wrapper: ReplicaAccountInfoVersions,
        slot: Slot,
        _is_startup: bool,
    ) -> GeyserPluginResult<()> {
        match msg_wrapper {
            ReplicaAccountInfoVersions::V0_0_1(_) => Ok(()), // Ignore
            ReplicaAccountInfoVersions::V0_0_2(_) => Ok(()), // Ignore
            ReplicaAccountInfoVersions::V0_0_3(msg) => Ok(self
                .handle_account_update(slot, msg)
                .map_err(|error| GeyserPluginError::Custom(error.into()))?),
        }
    }
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
/// # Safety
///
/// This function returns the GeyserPluginPostgres pointer as trait GeyserPlugin.
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    Box::into_raw(Box::new(Plugin::new()))
}
