#[cfg(test)]
mod tests {
    use {
        chrono::Utc,
        common::{
            message::{ArbitrageMessage, ArbitrageSide, Message, SequenceId},
            serializer::{BinarySerializer, Serializer, BUFFER_SIZE},
            AmountType, PriceType,
        },
        memmap2::{MmapMut, MmapOptions},
        serde_json::Value,
        solana_sdk::{clock::Slot, pubkey::Pubkey, signature::Signature},
        std::{
            fs::{remove_file, OpenOptions},
            thread,
            time::Instant,
        },
        trader::{
            self,
            receiver::{MmapReceiver, Receiver},
        },
    };

    fn create_arbitrage(slot: Slot) -> ArbitrageMessage {
        ArbitrageMessage {
            sequence_id: SequenceId::new(),
            slot,
            buy_side_info: ArbitrageSide {
                lp_name: format!("Test Buy LP name {}", slot),
                lp_address: Pubkey::new_unique(),
                base_qty: slot as AmountType + 1.1,
                quote_amount: slot as AmountType + 1.2,
                price: slot as PriceType + 1.3,
                fee: slot as PriceType + 1.4,
                lp_last_upd_slot: slot + 1,
                lp_last_upd_txn_sig: Some(Signature::new_unique()),
            },
            sell_side_info: ArbitrageSide {
                lp_name: format!("Test Sell LP name {}", slot),
                lp_address: Pubkey::new_unique(),
                base_qty: slot as AmountType + 1.5,
                quote_amount: slot as AmountType + 1.6,
                price: slot as PriceType + 1.7,
                fee: slot as PriceType + 1.8,
                lp_last_upd_slot: slot + 2,
                lp_last_upd_txn_sig: Some(Signature::new_unique()),
            },
            swap_base_qty: slot as AmountType + 1.9,
            swap_buy_quote_amount: slot as AmountType + 2.1,
            swap_sell_quote_amount: slot as AmountType + 3.4,
        }
    }

    fn create_config(file_name: &str) -> Value {
        let config_str = format!(
            r#"
            {{
                "file_name": "{}"
            }}"#,
            file_name
        );
        serde_json::from_str(&config_str).unwrap()
    }

    fn create_sender(file_name: &str) -> MmapMut {
        let sender_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_name)
            .unwrap();
        sender_file.set_len(BUFFER_SIZE as u64).unwrap();
        unsafe { MmapOptions::new().len(BUFFER_SIZE).map_mut(&sender_file) }.unwrap()
    }

    #[test]
    fn test_binary_serializing_with_mmap_sequential() {
        let file_name = &format!("/tmp/mmap_file_{}", Utc::now().format("%Y-%m-%dT%H%M%S%f"));
        let mut sender = create_sender(file_name);
        let mut receiver: MmapReceiver<BinarySerializer> = MmapReceiver::new(&create_config(file_name));

        // The mmap file is no longer needed after creating recevier.
        remove_file(file_name).unwrap();

        let mut sender_serializer = BinarySerializer::new();

        let mut arb_buffer: Vec<u8> = vec![0; BUFFER_SIZE];
        let total = 10_000;
        let start = Instant::now();
        for slot in 1..total {
            let arb_src = create_arbitrage(slot);

            sender_serializer
                .serialize_arbitrage(&arb_src, &mut arb_buffer)
                .unwrap();
            sender.copy_from_slice(&arb_buffer[..]);

            let arb_msg = receiver.receive().unwrap().unwrap();
            if let Message::Arbitrage(arb_res) = arb_msg {
                assert_eq!(arb_src, arb_res);
            } else {
                panic!("Unexpected message {}!", arb_msg);
            }
        }
        println!(
            "Time duration for creating, sending and deserializing {total} arbs sequentially is {} us.",
            start.elapsed().as_micros()
        );
    }

    #[test]
    fn test_binary_serializing_with_mmap_concurrent() {
        let file_name = format!("/tmp/mmap_file_{}", Utc::now().format("%Y-%m-%dT%H%M%S%f"));
        let mut sender = create_sender(&file_name);

        let mut receiver: Option<MmapReceiver<BinarySerializer>> = None;
        while receiver.is_none() {
            receiver = MmapReceiver::try_new(&create_config(&file_name));
        }
        let mut receiver = receiver.unwrap();

        // The mmap file is no longer needed after creating recevier.
        remove_file(&file_name).unwrap();

        let total = 10_000;
        let start = Instant::now();

        let sender_thread = thread::spawn(move || {
            let mut sender_serializer = BinarySerializer::new();
            let mut arb_buffer: Vec<u8> = vec![0; BUFFER_SIZE];

            for slot in 1..=total {
                let arb = create_arbitrage(slot);
                sender_serializer
                    .serialize_arbitrage(&arb, &mut arb_buffer)
                    .unwrap();
                /*
                                println!(
                                    "{} sending {slot}",
                                    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros()
                                );
                */
                sender.copy_from_slice(&arb_buffer[..]);
            }
        });

        let receiver_thread = thread::spawn(move || {
            let mut slot: Slot = 0;
            while slot < total {
                if let Ok(Some(Message::Arbitrage(arb))) = receiver.receive() {
                    /*
                                        println!(
                                            "{} received {}",
                                            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros(),
                                            arb.slot
                                        );
                    */
                    slot = arb.slot;
                }
            }
        });

        sender_thread.join().unwrap();
        receiver_thread.join().unwrap();

        println!(
            "Time duration for creating, sending and deserializing {total} arbs concurrently is {} us.",
            start.elapsed().as_micros()
        );
    }
}
