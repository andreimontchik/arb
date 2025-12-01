#[cfg(test)]
mod tests {
    use {
        common::{
            message::{ArbitrageMessage, SequenceId},
            serializer::CsvSerializer,
            test_util::tests::create_liquidity_pool,
            AccountType, AmountType, LiquidityPoolState,
        },
        plugin::arbitrage::{ArbitrageExecutor, ArbitragePersister},
        serde_json::Value,
        solana_sdk::signature::Signature,
    };

    #[test]
    fn test_perist_arbitrage() {
        let mut arb_file = std::env::temp_dir();
        arb_file.push("arb");
        let config_str = format!(
            r#"
            {{
                "file_name": "{}"
            }}"#,
            arb_file.display()
        );
        let config: Value = serde_json::from_str(&config_str).unwrap();
        let mut persister: ArbitragePersister<CsvSerializer> = ArbitragePersister::new(config);

        for i in 0..513 {
            let mut arb = ArbitrageMessage::new(
                SequenceId::new(),
                1,
                &LiquidityPoolState::new(create_liquidity_pool(AccountType::RaydiumAmmPoolAccount)),
                &LiquidityPoolState::new(create_liquidity_pool(AccountType::RaydiumAmmPoolAccount)),
                (i * 7) as AmountType,
                (i * 11) as AmountType,
                (i * 11) as AmountType,
            );
            if i % 2 == 0 {
                arb.buy_side_info.lp_last_upd_slot = i;
            } else {
                arb.sell_side_info.lp_last_upd_slot = i;
            }
            arb.buy_side_info.lp_last_upd_txn_sig = Some(Signature::new_unique());
            arb.sell_side_info.lp_last_upd_txn_sig = Some(Signature::new_unique());
            persister.execute(&arb).unwrap();
        }
    }
}
