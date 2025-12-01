pub mod arbitrage;
mod config;
mod plugin;
pub mod processor;

use {
    common::metrics::MetricKey,
    solana_sdk::clock::{Slot, UnixTimestamp},
    std::{collections::HashMap, hash::Hash},
};

#[derive(Debug)]
pub struct BlockState {
    pub slot: Slot,
    pub block_time: UnixTimestamp,
    pub block_height: u64,
}

#[derive(Debug)]

pub struct LastUpdateSlotCache<T>
where
    T: Eq + Hash,
{
    updates: HashMap<T, Slot>,
}

impl<T> LastUpdateSlotCache<T>
where
    T: Eq + Hash,
{
    fn new() -> Self {
        LastUpdateSlotCache {
            updates: HashMap::new(),
        }
    }

    fn is_new_update(&self, slot: Slot, entry: &T) -> bool {
        self.updates
            .get(entry)
            .map(|last_slot| slot >= *last_slot)
            .unwrap_or(true)
    }

    fn update(&mut self, slot: Slot, entry: T) {
        self.updates.insert(entry, slot);
    }
}

enum PluginMetricKey {
    AccountUpdateDuration,
    ArbitrageEvaluateDuration,
    ArbitrageProcessDuration,
}

impl MetricKey for PluginMetricKey {
    fn to_str(&self) -> &str {
        match self {
            PluginMetricKey::AccountUpdateDuration => "account-update-us",
            PluginMetricKey::ArbitrageEvaluateDuration => "arbitrage-evaluate-us",
            PluginMetricKey::ArbitrageProcessDuration => "arbitrage-process-us",
        }
    }
}

#[cfg(test)]
mod tests {
    use {super::*, solana_sdk::pubkey::Pubkey};

    #[test]
    fn test_last_updated_slot_cache() {
        let mut cache: LastUpdateSlotCache<Pubkey> = LastUpdateSlotCache::new();

        let address = Pubkey::new_unique();
        assert!(cache.is_new_update(1, &address));

        cache.update(2, Pubkey::new_unique());
        assert!(cache.is_new_update(1, &address));

        cache.update(1, address);
        assert!(cache.is_new_update(1, &address));
        assert!(cache.is_new_update(2, &address));

        cache.update(2, address);
        assert!(!cache.is_new_update(1, &address));
        assert!(cache.is_new_update(2, &address));
    }
}
