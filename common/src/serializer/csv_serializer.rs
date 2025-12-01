use {
    super::{
        bytes_to_decimal_string, write_str_to_buffer, MessageType, Result, Serializer, SerializerError,
        COMMA_CHAR, COMMA_CHAR_CODE, EMPTY_STRING,
    },
    crate::{
        message::{
            AccountUpdateMessage, ArbitrageMessage, ArbitrageSide, BlockUpdateMessage, SequenceId,
            SwapMessage, SwapType,
        },
        AmountType, LiquidityPool, PriceType,
    },
    anyhow::bail,
    solana_sdk::{
        bs58,
        clock::{Slot, UnixTimestamp},
        pubkey::Pubkey,
        signature::Signature,
    },
    std::{
        num::{ParseFloatError, ParseIntError},
        str::{self, FromStr},
    },
};

pub struct CsvSerializer {}

impl Serializer for CsvSerializer {
    fn new() -> Self {
        CsvSerializer {}
    }

    fn serialize_liquidity_pool(&mut self, lp: &LiquidityPool, buffer: &mut Vec<u8>) -> Result<usize> {
        let mut offset: usize = 0;
        for account_info in LiquidityPool::accounts(&lp) {
            offset = write_str_to_buffer(
                &format!(
                    "{},{:?},{:?},",
                    MessageType::LiquidityPoolConfiguration.to_string(),
                    account_info.1,
                    account_info.0,
                ),
                buffer,
                offset,
            );
        }
        Ok(offset)
    }

    fn serialize_block_update(
        &mut self,
        msg: &BlockUpdateMessage,
        buffer: &mut Vec<u8>,
    ) -> Result<usize> {
        Ok(write_str_to_buffer(
            &format!(
                "{},{},{},{},",
                MessageType::BlockUpdate.to_string(),
                msg.slot,
                msg.block_time
                    .map(|time| time.to_string())
                    .unwrap_or(EMPTY_STRING.to_string()),
                msg.block_height
                    .map(|height| height.to_string())
                    .unwrap_or(EMPTY_STRING.to_string()),
            ),
            buffer,
            0,
        ))
    }

    fn serialize_account_update(
        &mut self,
        msg: &AccountUpdateMessage,
        buffer: &mut Vec<u8>,
    ) -> Result<usize> {
        Ok(write_str_to_buffer(
            &format!(
                "{},{},{},{},{},",
                MessageType::AccountUpdate.to_string(),
                msg.slot,
                msg.address,
                bs58::encode(&msg.data).into_string(),
                msg.txn_signature
                    .map(|sig| sig.to_string())
                    .unwrap_or(EMPTY_STRING.to_string())
            ),
            buffer,
            0,
        ))
    }

    fn serialize_arbitrage(
        &mut self,
        arbitrage: &ArbitrageMessage,
        buffer: &mut Vec<u8>,
    ) -> Result<usize> {
        Ok(write_str_to_buffer(
            &format!(
                "{},{},{},\"{}\",{},{},{},{},{},{},{},\"{}\",{},{},{},{},{},{},{},{},{},{},",
                MessageType::Arbitrage.to_string(),
                arbitrage.sequence_id,
                arbitrage.slot,
                arbitrage.buy_side_info.lp_name,
                arbitrage.buy_side_info.lp_address,
                arbitrage.buy_side_info.base_qty,
                arbitrage.buy_side_info.quote_amount,
                arbitrage.buy_side_info.price,
                arbitrage.buy_side_info.fee,
                arbitrage.buy_side_info.lp_last_upd_slot,
                arbitrage
                    .buy_side_info
                    .lp_last_upd_txn_sig
                    .map(|sig| sig.to_string())
                    .unwrap_or(EMPTY_STRING.to_string()),
                arbitrage.sell_side_info.lp_name,
                arbitrage.sell_side_info.lp_address,
                arbitrage.sell_side_info.base_qty,
                arbitrage.sell_side_info.quote_amount,
                arbitrage.sell_side_info.price,
                arbitrage.sell_side_info.fee,
                arbitrage.sell_side_info.lp_last_upd_slot,
                arbitrage
                    .sell_side_info
                    .lp_last_upd_txn_sig
                    .map(|sig| sig.to_string())
                    .unwrap_or(EMPTY_STRING.to_string()),
                arbitrage.swap_base_qty,
                arbitrage.swap_buy_quote_amount,
                arbitrage.swap_sell_quote_amount,
            ),
            buffer,
            0,
        ))
    }

    fn serialize_swap(&mut self, msg: &SwapMessage, buffer: &mut Vec<u8>) -> Result<usize> {
        Ok(write_str_to_buffer(
            &format!(
                "{},{},\"{}\",{},{},{},{},",
                MessageType::Swap.to_string(),
                msg.sequence_id,
                msg.lp_name,
                msg.lp_address,
                msg.swap_type.to_string(),
                msg.base_qty,
                msg.quote_amount,
            ),
            buffer,
            0,
        ))
    }

    fn deserialize_message_type(&self, buffer: &[u8]) -> Result<MessageType> {
        // Identify message type
        let first_comma_position = buffer
            .iter()
            .position(|&byte| byte == COMMA_CHAR_CODE)
            .unwrap_or(0);
        if first_comma_position == 0 {
            bail!(SerializerError::InvalidBuffer {
                error: "Failed to find message type.".to_string(),
            });
        }

        let msg_type_str = str::from_utf8(&buffer[0..first_comma_position]).map_err(|err| {
            SerializerError::InvalidBuffer {
                error: format!(
                    "Failed to parse message type from {}! {}",
                    bytes_to_decimal_string(&buffer[0..first_comma_position]),
                    err.to_string()
                ),
            }
        })?;

        MessageType::try_from(msg_type_str)
    }

    fn deserialize_block_update(&mut self, buffer: &[u8]) -> Result<BlockUpdateMessage> {
        let msg: &str = str::from_utf8(buffer).map_err(|err| SerializerError::InvalidBuffer {
            error: err.to_string(),
        })?;

        let msg_vec: Vec<&str> = msg.split(COMMA_CHAR).collect();
        if msg_vec.len() < 5 {
            bail!(SerializerError::InvalidMessageFormat {
                msg: format!("{:?}", msg_vec),
            });
        }

        let slot: Slot =
            msg_vec[1]
                .parse()
                .map_err(|err: ParseIntError| SerializerError::ParseError {
                    msg: format!("Invalid slot {:?}", msg_vec[1].to_string()),
                    error: err.to_string(),
                })?;

        let block_time: Option<UnixTimestamp> = if !msg_vec[2].is_empty() {
            Some(
                msg_vec[2]
                    .parse()
                    .map_err(|err: ParseIntError| SerializerError::ParseError {
                        msg: format!("Invalid block time {:?}", msg_vec[2].to_string()),
                        error: err.to_string(),
                    })?,
            )
        } else {
            None
        };

        let block_height: Option<u64> = if !msg_vec[3].is_empty() {
            Some(
                msg_vec[3]
                    .parse()
                    .map_err(|err: ParseIntError| SerializerError::ParseError {
                        msg: format!("Invalid block height {:?}", msg_vec[3].to_string()),
                        error: err.to_string(),
                    })?,
            )
        } else {
            None
        };

        Ok(BlockUpdateMessage {
            slot,
            block_time,
            block_height,
        })
    }

    fn deserialize_account_update(&mut self, buffer: &[u8]) -> Result<AccountUpdateMessage> {
        let msg: &str = str::from_utf8(buffer).map_err(|err| SerializerError::InvalidBuffer {
            error: err.to_string(),
        })?;

        let msg_vec: Vec<&str> = msg.split(COMMA_CHAR).collect();
        if msg_vec.len() < 6 {
            bail!(SerializerError::InvalidMessageFormat {
                msg: format!("{:?}", msg_vec),
            });
        }

        let slot: Slot =
            msg_vec[1]
                .parse()
                .map_err(|err: ParseIntError| SerializerError::ParseError {
                    msg: format!("Invalid slot {:?}", msg_vec[1].to_string()),
                    error: err.to_string(),
                })?;

        let address = Pubkey::from_str(msg_vec[2]).map_err(|err| SerializerError::ParseError {
            msg: format!("Invalid address {:?}", msg_vec[2].to_string()),
            error: err.to_string(),
        })?;

        let data = bs58::decode(msg_vec[3])
            .into_vec()
            .map_err(|err| SerializerError::ParseError {
                msg: format!("Invalid data {:?}", msg_vec[3].to_string()),
                error: err.to_string(),
            })?;

        let txn_signature = if !msg_vec[4].is_empty() {
            Some(
                Signature::from_str(msg_vec[4]).map_err(|err| SerializerError::ParseError {
                    msg: format!("Invalid signature {:?}", msg_vec[4].to_string()),
                    error: err.to_string(),
                })?,
            )
        } else {
            None
        };

        Ok(AccountUpdateMessage {
            slot,
            address,
            data,
            txn_signature,
        })
    }

    fn deserialize_arbitrage(&mut self, buffer: &[u8]) -> Result<ArbitrageMessage> {
        let msg = str::from_utf8(buffer).map_err(|err| SerializerError::InvalidBuffer {
            error: err.to_string(),
        })?;

        let msg_vec: Vec<&str> = msg.split(COMMA_CHAR).collect();
        if msg_vec.len() < 22 {
            bail!(SerializerError::InvalidMessageFormat {
                msg: format!("{:?}", msg_vec),
            });
        };

        let sequence_id: SequenceId =
            SequenceId::from_str(msg_vec[1]).map_err(|error| SerializerError::ParseError {
                msg: format!("Invalid sequence id {:?}", msg_vec[1].to_string()),
                error,
            })?;

        let slot: Slot =
            msg_vec[2]
                .parse()
                .map_err(|err: ParseIntError| SerializerError::ParseError {
                    msg: format!("Invalid slot {:?}", msg_vec[1].to_string()),
                    error: err.to_string(),
                })?;

        let buy_name = msg_vec[3].replace("\"", "");

        let buy_address = Pubkey::from_str(msg_vec[4]).map_err(|err| SerializerError::ParseError {
            msg: format!("Invalid buy address {:?}", msg_vec[4].to_string()),
            error: err.to_string(),
        })?;

        let buy_base_qty: AmountType =
            msg_vec[5]
                .parse()
                .map_err(|err: ParseFloatError| SerializerError::ParseError {
                    msg: format!("Invalid buy base qty {:?}", msg_vec[5].to_string()),
                    error: err.to_string(),
                })?;

        let buy_quote_amount: AmountType =
            msg_vec[6]
                .parse()
                .map_err(|err: ParseFloatError| SerializerError::ParseError {
                    msg: format!("Invalid buy quote qty {:?}", msg_vec[6].to_string()),
                    error: err.to_string(),
                })?;

        let buy_price: PriceType =
            msg_vec[7]
                .parse()
                .map_err(|error: ParseFloatError| SerializerError::ParseError {
                    msg: format!("buy price slot {:?}", msg_vec[7].to_string()),
                    error: error.to_string(),
                })?;

        let buy_fee: PriceType =
            msg_vec[8]
                .parse()
                .map_err(|error: ParseFloatError| SerializerError::ParseError {
                    msg: format!("Invalid buy fee {:?}", msg_vec[8].to_string()),
                    error: error.to_string(),
                })?;

        let buy_lp_last_upd_slot: Slot =
            msg_vec[9]
                .parse()
                .map_err(|err: ParseIntError| SerializerError::ParseError {
                    msg: format!("Invalid buy last update slot {:?}", msg_vec[9].to_string()),
                    error: err.to_string(),
                })?;

        let buy_lp_last_upd_txn_sig: Option<Signature> = if !msg_vec[10].is_empty() {
            Some(
                Signature::from_str(msg_vec[10]).map_err(|err| SerializerError::ParseError {
                    msg: format!(
                        "Invalid buy last update txn signature {:?}",
                        msg_vec[9].to_string()
                    ),
                    error: err.to_string(),
                })?,
            )
        } else {
            None
        };

        let sell_lp_name = msg_vec[11].replace("\"", "");

        let sell_lp_address =
            Pubkey::from_str(msg_vec[12]).map_err(|err| SerializerError::ParseError {
                msg: format!("Invalid sell LP address {:?}", msg_vec[12].to_string()),
                error: err.to_string(),
            })?;

        let sell_base_qty: AmountType =
            msg_vec[13]
                .parse()
                .map_err(|error: ParseFloatError| SerializerError::ParseError {
                    msg: format!("Invalid sell base qty {:?}", msg_vec[13].to_string()),
                    error: error.to_string(),
                })?;

        let sell_quote_amount: AmountType =
            msg_vec[14]
                .parse()
                .map_err(|error: ParseFloatError| SerializerError::ParseError {
                    msg: format!("Invalid sell quote qty {:?}", msg_vec[14].to_string()),
                    error: error.to_string(),
                })?;

        let sell_price: PriceType =
            msg_vec[15]
                .parse()
                .map_err(|error: ParseFloatError| SerializerError::ParseError {
                    msg: format!("Invalid sell price {:?}", msg_vec[15].to_string()),
                    error: error.to_string(),
                })?;

        let sell_fee: PriceType =
            msg_vec[16]
                .parse()
                .map_err(|error: ParseFloatError| SerializerError::ParseError {
                    msg: format!("Invalid sell fee {:?}", msg_vec[16].to_string()),
                    error: error.to_string(),
                })?;

        let sell_lp_last_upd_slot: Slot =
            msg_vec[17]
                .parse()
                .map_err(|err: ParseIntError| SerializerError::ParseError {
                    msg: format!("Invalid sell LP las upd slot {:?}", msg_vec[17].to_string()),
                    error: err.to_string(),
                })?;

        let sell_lp_last_upd_txn_sig: Option<Signature> = if !msg_vec[18].is_empty() {
            Some(
                Signature::from_str(msg_vec[18]).map_err(|err| SerializerError::ParseError {
                    msg: format!(
                        "Invalid sell LP last upd txn signature {:?}",
                        msg_vec[17].to_string()
                    ),
                    error: err.to_string(),
                })?,
            )
        } else {
            None
        };

        let swap_base_qty: AmountType =
            msg_vec[19]
                .parse()
                .map_err(|error: ParseFloatError| SerializerError::ParseError {
                    msg: format!("Invalid swap base qty {:?}", msg_vec[19].to_string()),
                    error: error.to_string(),
                })?;

        let swap_buy_quote_amount: AmountType =
            msg_vec[20]
                .parse()
                .map_err(|error: ParseFloatError| SerializerError::ParseError {
                    msg: format!("Invalid swap buy quote amount {:?}", msg_vec[20].to_string()),
                    error: error.to_string(),
                })?;

        let swap_sell_quote_amount: AmountType =
            msg_vec[21]
                .parse()
                .map_err(|error: ParseFloatError| SerializerError::ParseError {
                    msg: format!("Invalid swap sell quote amount {:?}", msg_vec[21].to_string()),
                    error: error.to_string(),
                })?;

        Ok(ArbitrageMessage {
            sequence_id,
            slot,
            buy_side_info: ArbitrageSide {
                lp_name: buy_name,
                lp_address: buy_address,
                base_qty: buy_base_qty,
                quote_amount: buy_quote_amount,
                price: buy_price,
                fee: buy_fee,
                lp_last_upd_slot: buy_lp_last_upd_slot,
                lp_last_upd_txn_sig: buy_lp_last_upd_txn_sig,
            },
            sell_side_info: ArbitrageSide {
                lp_name: sell_lp_name,
                lp_address: sell_lp_address,
                base_qty: sell_base_qty,
                quote_amount: sell_quote_amount,
                price: sell_price,
                fee: sell_fee,
                lp_last_upd_slot: sell_lp_last_upd_slot,
                lp_last_upd_txn_sig: sell_lp_last_upd_txn_sig,
            },
            swap_base_qty,
            swap_buy_quote_amount,
            swap_sell_quote_amount,
        })
    }

    fn deserialize_swap(&mut self, buffer: &[u8]) -> Result<SwapMessage> {
        let msg = str::from_utf8(buffer).map_err(|err| SerializerError::InvalidBuffer {
            error: err.to_string(),
        })?;

        let msg_vec: Vec<&str> = msg.split(COMMA_CHAR).collect();
        if msg_vec.len() < 7 {
            bail!(SerializerError::InvalidMessageFormat {
                msg: format!("{:?}", msg_vec),
            });
        };

        let sequence_id: SequenceId =
            SequenceId::from_str(msg_vec[1]).map_err(|error| SerializerError::ParseError {
                msg: format!("Invalid sequence id {:?}", msg_vec[1].to_string()),
                error,
            })?;

        let lp_name = msg_vec[2].replace("\"", "");

        let lp_address = Pubkey::from_str(msg_vec[3]).map_err(|err| SerializerError::ParseError {
            msg: format!("Invalid LP address {:?}", msg_vec[3].to_string()),
            error: err.to_string(),
        })?;

        let swap_type: SwapType =
            SwapType::try_from(msg_vec[4]).map_err(|err| SerializerError::ParseError {
                msg: format!("Invalid Swap type {:?}", msg_vec[4]),
                error: err.to_string(),
            })?;

        let base_qty: AmountType =
            msg_vec[5]
                .parse()
                .map_err(|error: ParseFloatError| SerializerError::ParseError {
                    msg: format!("Invalid base qty {:?}", msg_vec[5].to_string()),
                    error: error.to_string(),
                })?;
        let quote_amount: AmountType =
            msg_vec[6]
                .parse()
                .map_err(|error: ParseFloatError| SerializerError::ParseError {
                    msg: format!("Invalid quote amount {:?}", msg_vec[6].to_string()),
                    error: error.to_string(),
                })?;

        Ok(SwapMessage {
            sequence_id,
            lp_name,
            lp_address,
            swap_type,
            base_qty,
            quote_amount,
        })
    }
}
