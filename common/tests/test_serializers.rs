#[cfg(test)]
mod tests {
    use {
        common::{
            message::{
                AccountUpdateMessage, ArbitrageMessage, BlockUpdateMessage, Message, SequenceId,
                SwapMessage, SwapType,
            },
            serializer::{BinarySerializer, CsvSerializer, Serializer, BUFFER_SIZE},
            test_util::tests::{create_liquidity_pool, MOCK_wSOL_TOKEN, MOCK_USDC_TOKEN},
            AccountType, AmountType, LiquidityGroupCode, LiquidityPool, LiquidityPoolState,
            RaydiumAmmLp,
        },
        rand::Rng,
        solana_sdk::{clock::Slot, pubkey::Pubkey, signature::Signature},
        std::ops::Mul,
    };

    fn test_serializing_block_update<T: Serializer>(serializer: &mut T) {
        let mut buffer: Vec<u8> = vec![0; BUFFER_SIZE];
        for i in 0..513 {
            let src_msg = Message::BlockUpdate(BlockUpdateMessage {
                slot: i,
                block_time: Some(i.mul(100).try_into().unwrap()),
                block_height: Some(i * 1000),
            });
            assert!(serializer.serialize_message(&src_msg, &mut buffer).is_ok());
            let dst_msg = serializer.deserialize_message(&buffer).unwrap();
            assert_eq!(src_msg, dst_msg);
        }
    }

    #[test]
    fn test_csv_serializing_block_update() {
        let mut serializer = CsvSerializer::new();
        test_serializing_block_update(&mut serializer);
    }

    fn test_serializing_account_update<T: Serializer>(serializer: &mut T) {
        let mut buffer: Vec<u8> = vec![0; BUFFER_SIZE];
        let mut rng = rand::thread_rng();
        for i in 0..513 {
            let data: Vec<u8> = (0..i + 10).map(|_| rng.gen()).collect();
            let src_msg = Message::AccountUpdate(AccountUpdateMessage {
                slot: i as Slot,
                address: Pubkey::new_unique(),
                data,
                txn_signature: Some(Signature::new_unique()),
            });
            assert!(serializer.serialize_message(&src_msg, &mut buffer).is_ok());
            let dst_msg = serializer.deserialize_message(&buffer).unwrap();
            assert_eq!(src_msg, dst_msg);
        }
    }

    #[test]
    fn test_csv_serializing_account_update() {
        let mut serializer = CsvSerializer::new();
        test_serializing_account_update(&mut serializer);
    }

    fn test_serializing_account_configuration<T: Serializer>(serializer: &mut T) {
        let mut buffer: Vec<u8> = vec![0; BUFFER_SIZE];
        for i in 0..255 {
            let src_msg = LiquidityPool::RaydiumAmm(RaydiumAmmLp {
                liquidity_group: LiquidityGroupCode::SOL_USD,
                program_id: Pubkey::new_unique(),
                name: format!("RaydiumAmm{i}"),
                address: Pubkey::new_unique(),
                authority: Pubkey::new_unique(),
                base_token: MOCK_wSOL_TOKEN,
                base_token_vault: Pubkey::new_unique(),
                min_base_token_balance: 0.01,
                quote_token: MOCK_USDC_TOKEN,
                quote_token_vault: Pubkey::new_unique(),
                min_quote_token_balance: 1.0,
            });
            assert!(serializer.serialize_liquidity_pool(&src_msg, &mut buffer).is_ok());
        }
    }

    #[test]
    fn test_csv_serializing_account_configuration() {
        let mut serializer = CsvSerializer::new();
        test_serializing_account_configuration(&mut serializer);
    }

    fn test_serializing_arbitrage<T: Serializer>(serializer: &mut T) {
        let mut buffer: Vec<u8> = vec![0; BUFFER_SIZE];

        for i in 0..513 {
            let mut buy_lp_state =
                LiquidityPoolState::new(create_liquidity_pool(AccountType::RaydiumAmmPoolAccount));
            buy_lp_state.latest_upd_slot = i;
            buy_lp_state.latest_upd_txn_sig = if i % 2 == 0 {
                Some(Signature::new_unique())
            } else {
                None
            };

            let mut sell_lp_state =
                LiquidityPoolState::new(create_liquidity_pool(AccountType::RaydiumAmmPoolAccount));
            sell_lp_state.latest_upd_slot = i + 1;
            sell_lp_state.latest_upd_txn_sig = if i % 2 != 0 {
                Some(Signature::new_unique())
            } else {
                None
            };

            let src_arb = ArbitrageMessage::new(
                SequenceId::new(),
                1,
                &buy_lp_state,
                &sell_lp_state,
                (i * 7) as AmountType,
                (i * 11) as AmountType,
                (i * 12) as AmountType,
            );
            serializer.serialize_arbitrage(&src_arb, &mut buffer).unwrap();

            let des_arb = serializer.deserialize_arbitrage(&buffer).unwrap();
            assert_eq!(src_arb, des_arb);
        }
    }

    #[test]
    fn test_binary_serializing_arbitrage() {
        let mut serializer = BinarySerializer::new();
        test_serializing_arbitrage(&mut serializer);
    }

    #[test]
    fn test_csv_serializing_arbitrage() {
        let mut serializer = CsvSerializer::new();
        test_serializing_arbitrage(&mut serializer);
    }

    fn test_serializing_swap<T: Serializer>(serializer: &mut T) {
        let mut buffer: Vec<u8> = vec![0; BUFFER_SIZE];

        for i in 0..513 {
            let mut sequence_id = SequenceId::new();
            let lp = create_liquidity_pool(AccountType::RaydiumAmmPoolAccount);
            let swap_type = if i & 2 == 0 { SwapType::Buy } else { SwapType::Sell };
            let base_qty: AmountType = 100000000.0 + i as AmountType;
            let quote_amount: AmountType = 10000000.0 + i as AmountType;

            let msg = SwapMessage {
                sequence_id: sequence_id.increment_and_get(),
                lp_name: LiquidityPool::name(&lp).to_string(),
                lp_address: *LiquidityPool::address(&lp),
                swap_type,
                base_qty,
                quote_amount,
            };

            serializer.serialize_swap(&msg, &mut buffer).unwrap();
            let des_msg = serializer.deserialize_swap(&buffer).unwrap();
            assert_eq!(msg, des_msg);
        }
    }

    #[test]
    fn test_binary_serializing_swap() {
        let mut serializer = BinarySerializer::new();
        test_serializing_swap(&mut serializer);
    }

    #[test]
    fn test_csv_serializing_swap() {
        let mut serializer = CsvSerializer::new();
        test_serializing_swap(&mut serializer);
    }
}
