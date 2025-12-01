use {
    super::{
        bytes_to_decimal_string, try_write_str_to_slice, MessageType, Result, Serializer,
        SerializerError, BUFFER_SIZE, ZERO_BYTE,
    },
    crate::{
        message::{ArbitrageMessage, ArbitrageSide, SequenceId, SwapMessage, SwapType},
        AmountType, PriceType,
    },
    anyhow::bail,
    solana_sdk::{clock::Slot, pubkey::Pubkey, signature::Signature},
    std::array::TryFromSliceError,
};

pub struct BinarySerializer {
    inner_buffer: [u8; BUFFER_SIZE],
}

impl Serializer for BinarySerializer {
    fn new() -> Self {
        Self {
            inner_buffer: [ZERO_BYTE; BUFFER_SIZE],
        }
    }

    fn serialize_arbitrage(
        &mut self,
        arbitrage: &ArbitrageMessage,
        buffer: &mut Vec<u8>,
    ) -> Result<usize> {
        if buffer.len() != self.inner_buffer.len() {
            bail!(SerializerError::InvalidBufferSize {
                size: buffer.len(),
                expected: self.inner_buffer.len(),
            });
        }

        self.inner_buffer[0] = MessageType::Arbitrage as u8;
        self.inner_buffer[1..13].copy_from_slice(&arbitrage.sequence_id.to_bytes());
        self.inner_buffer[13..21].copy_from_slice(&arbitrage.slot.to_le_bytes());
        try_write_str_to_slice(&arbitrage.buy_side_info.lp_name, &mut self.inner_buffer[21..53])?;
        self.inner_buffer[53..85].copy_from_slice(&arbitrage.buy_side_info.lp_address.to_bytes());
        self.inner_buffer[85..93].copy_from_slice(&arbitrage.buy_side_info.base_qty.to_le_bytes());
        self.inner_buffer[93..101].copy_from_slice(&arbitrage.buy_side_info.quote_amount.to_le_bytes());
        self.inner_buffer[101..109].copy_from_slice(&arbitrage.buy_side_info.price.to_le_bytes());
        self.inner_buffer[109..117].copy_from_slice(&arbitrage.buy_side_info.fee.to_le_bytes());
        self.inner_buffer[117..125]
            .copy_from_slice(&arbitrage.buy_side_info.lp_last_upd_slot.to_le_bytes());
        if let Some(txn_sig) = arbitrage.buy_side_info.lp_last_upd_txn_sig {
            self.inner_buffer[125..189].copy_from_slice(txn_sig.as_ref())
        } else {
            self.inner_buffer[125..189].fill(0)
        }
        try_write_str_to_slice(
            &arbitrage.sell_side_info.lp_name,
            &mut self.inner_buffer[189..231],
        )?;
        self.inner_buffer[231..263].copy_from_slice(&arbitrage.sell_side_info.lp_address.to_bytes());
        self.inner_buffer[263..271].copy_from_slice(&arbitrage.sell_side_info.base_qty.to_le_bytes());
        self.inner_buffer[271..279]
            .copy_from_slice(&arbitrage.sell_side_info.quote_amount.to_le_bytes());
        self.inner_buffer[279..287].copy_from_slice(&arbitrage.sell_side_info.price.to_le_bytes());
        self.inner_buffer[287..295].copy_from_slice(&arbitrage.sell_side_info.fee.to_le_bytes());
        self.inner_buffer[295..303]
            .copy_from_slice(&arbitrage.sell_side_info.lp_last_upd_slot.to_le_bytes());
        if let Some(txn_sig) = arbitrage.sell_side_info.lp_last_upd_txn_sig {
            self.inner_buffer[303..367].copy_from_slice(txn_sig.as_ref())
        } else {
            self.inner_buffer[303..367].fill(0)
        }
        self.inner_buffer[367..375].copy_from_slice(&arbitrage.swap_base_qty.to_le_bytes());
        self.inner_buffer[375..383].copy_from_slice(&arbitrage.swap_buy_quote_amount.to_le_bytes());
        self.inner_buffer[383..391].copy_from_slice(&arbitrage.swap_sell_quote_amount.to_le_bytes());

        buffer.copy_from_slice(&self.inner_buffer);

        Ok(self.inner_buffer.len())
    }

    fn serialize_swap(
        &mut self,
        msg: &crate::message::SwapMessage,
        buffer: &mut Vec<u8>,
    ) -> Result<usize> {
        if buffer.len() != self.inner_buffer.len() {
            bail!(SerializerError::InvalidBufferSize {
                size: buffer.len(),
                expected: self.inner_buffer.len(),
            });
        }

        self.inner_buffer[0] = MessageType::Swap as u8;
        self.inner_buffer[1..13].copy_from_slice(&msg.sequence_id.to_bytes());
        try_write_str_to_slice(&msg.lp_name, &mut self.inner_buffer[13..45])?;
        self.inner_buffer[45..77].copy_from_slice(&msg.lp_address.to_bytes());
        self.inner_buffer[77] = msg.swap_type as u8;
        self.inner_buffer[78..86].copy_from_slice(&msg.base_qty.to_le_bytes());
        self.inner_buffer[86..94].copy_from_slice(&msg.quote_amount.to_le_bytes());
        buffer.copy_from_slice(&self.inner_buffer);

        Ok(self.inner_buffer.len())
    }

    fn deserialize_message_type(&self, buffer: &[u8]) -> Result<MessageType> {
        if buffer.is_empty() {
            bail!(SerializerError::InvalidBuffer {
                error: "Buffer is empty.".to_string(),
            });
        }

        MessageType::try_from(buffer[0])
    }

    fn deserialize_arbitrage(&mut self, buffer: &[u8]) -> Result<ArbitrageMessage> {
        if buffer.len() != self.inner_buffer.len() {
            bail!(SerializerError::InvalidBufferSize {
                size: buffer.len(),
                expected: self.inner_buffer.len(),
            });
        }

        self.inner_buffer.copy_from_slice(buffer);

        let sequence_id = SequenceId::from_bytes(&self.inner_buffer[1..13])?;

        let slot: Slot = u64::from_le_bytes(self.inner_buffer[13..21].try_into().map_err(
            |err: TryFromSliceError| SerializerError::DecodeError {
                msg: format!(
                    "Invalid slot {:?}",
                    bytes_to_decimal_string(&self.inner_buffer[13..21])
                ),
                error: err.to_string(),
            },
        )?);

        let buy_lp_name = std::str::from_utf8(&self.inner_buffer[21..53])
            .map_err(|err| SerializerError::DecodeError {
                msg: format!(
                    "Invalid buy LP name {:?}",
                    bytes_to_decimal_string(&self.inner_buffer[21..53])
                ),
                error: err.to_string(),
            })?
            .trim();

        let buy_address = Pubkey::try_from(&self.inner_buffer[53..85]).map_err(|err| {
            SerializerError::DecodeError {
                msg: format!(
                    "Invalid buy address {:?}",
                    bytes_to_decimal_string(&self.inner_buffer[53..85])
                ),
                error: err.to_string(),
            }
        })?;

        let buy_base_qty: AmountType =
            f64::from_le_bytes(self.inner_buffer[85..93].try_into().map_err(
                |err: TryFromSliceError| SerializerError::DecodeError {
                    msg: format!(
                        "Invalid buy base qty {:?}",
                        bytes_to_decimal_string(&self.inner_buffer[85..93])
                    ),
                    error: err.to_string(),
                },
            )?);

        let buy_quote_amount: AmountType =
            f64::from_le_bytes(self.inner_buffer[93..101].try_into().map_err(
                |err: TryFromSliceError| SerializerError::DecodeError {
                    msg: format!(
                        "Invalid buy quote qty {:?}",
                        bytes_to_decimal_string(&self.inner_buffer[93..101])
                    ),
                    error: err.to_string(),
                },
            )?);

        let buy_price: PriceType = f64::from_le_bytes(self.inner_buffer[101..109].try_into().map_err(
            |err: TryFromSliceError| SerializerError::DecodeError {
                msg: format!(
                    "Invalid buy price {:?}",
                    bytes_to_decimal_string(&self.inner_buffer[101..109])
                ),
                error: err.to_string(),
            },
        )?);

        let buy_fee: PriceType = f64::from_le_bytes(self.inner_buffer[109..117].try_into().map_err(
            |err: TryFromSliceError| SerializerError::DecodeError {
                msg: format!(
                    "Invalid buy fee {:?}",
                    bytes_to_decimal_string(&self.inner_buffer[109..117])
                ),
                error: err.to_string(),
            },
        )?);

        let buy_lp_last_upd_slot: Slot =
            u64::from_le_bytes(self.inner_buffer[117..125].try_into().map_err(
                |err: TryFromSliceError| SerializerError::DecodeError {
                    msg: format!(
                        "Invalid buy LP last update slot {:?}",
                        bytes_to_decimal_string(&self.inner_buffer[117..125])
                    ),
                    error: err.to_string(),
                },
            )?);

        let buy_lp_last_upd_txn_sig =
            if self.inner_buffer[125..189].iter().any(|&byte| byte != ZERO_BYTE) {
                Some(
                    Signature::try_from(&self.inner_buffer[125..189]).map_err(|error| {
                        SerializerError::DecodeError {
                            msg: format!(
                                "Invalid buy LP last update signature {:?}",
                                bytes_to_decimal_string(&self.inner_buffer[125..189])
                            ),
                            error: error.to_string(),
                        }
                    })?,
                )
            } else {
                None
            };

        let sell_lp_name = std::str::from_utf8(&self.inner_buffer[189..221])
            .map_err(|err| SerializerError::DecodeError {
                msg: format!(
                    "Invalid sell LP name {:?}",
                    bytes_to_decimal_string(&self.inner_buffer[189..221])
                ),
                error: err.to_string(),
            })?
            .trim();

        let sell_lp_address = Pubkey::try_from(&self.inner_buffer[231..263]).map_err(|err| {
            SerializerError::DecodeError {
                msg: format!(
                    "Invalid sell LP address {:?}",
                    bytes_to_decimal_string(&self.inner_buffer[231..263])
                ),
                error: err.to_string(),
            }
        })?;

        let sell_base_qty: AmountType =
            f64::from_le_bytes(self.inner_buffer[263..271].try_into().map_err(
                |err: TryFromSliceError| SerializerError::DecodeError {
                    msg: format!(
                        "Invalid sell base qty {:?}",
                        bytes_to_decimal_string(&self.inner_buffer[263..271])
                    ),
                    error: err.to_string(),
                },
            )?);

        let sell_quote_amount: AmountType =
            f64::from_le_bytes(self.inner_buffer[271..279].try_into().map_err(
                |err: TryFromSliceError| SerializerError::DecodeError {
                    msg: format!(
                        "Invalid sell quote amount {:?}",
                        bytes_to_decimal_string(&self.inner_buffer[271..279])
                    ),
                    error: err.to_string(),
                },
            )?);

        let sell_price: PriceType = f64::from_le_bytes(self.inner_buffer[279..287].try_into().map_err(
            |err: TryFromSliceError| SerializerError::DecodeError {
                msg: format!(
                    "Invalid sell price {:?}",
                    bytes_to_decimal_string(&self.inner_buffer[289..287])
                ),
                error: err.to_string(),
            },
        )?);

        let sell_fee: PriceType = f64::from_le_bytes(self.inner_buffer[287..295].try_into().map_err(
            |err: TryFromSliceError| SerializerError::DecodeError {
                msg: format!(
                    "Invalid sell fee {:?}",
                    bytes_to_decimal_string(&self.inner_buffer[287..295])
                ),
                error: err.to_string(),
            },
        )?);

        let sell_lp_last_upd_slot: Slot =
            u64::from_le_bytes(self.inner_buffer[295..303].try_into().map_err(
                |err: TryFromSliceError| SerializerError::DecodeError {
                    msg: format!(
                        "Invalid sell last update slot {:?}",
                        bytes_to_decimal_string(&self.inner_buffer[295..303])
                    ),
                    error: err.to_string(),
                },
            )?);

        let sell_lp_last_upd_txn_sig =
            if self.inner_buffer[303..367].iter().any(|&byte| byte != ZERO_BYTE) {
                Some(
                    Signature::try_from(&self.inner_buffer[303..367]).map_err(|error| {
                        SerializerError::DecodeError {
                            msg: format!(
                                "Invalid sell LP last update txn signature {:?}",
                                bytes_to_decimal_string(&self.inner_buffer[303..367])
                            ),
                            error: error.to_string(),
                        }
                    })?,
                )
            } else {
                None
            };

        let swap_base_qty: AmountType =
            f64::from_le_bytes(self.inner_buffer[367..375].try_into().map_err(
                |err: TryFromSliceError| SerializerError::DecodeError {
                    msg: format!(
                        "Invalid swap base qty {:?}",
                        bytes_to_decimal_string(&self.inner_buffer[367..375])
                    ),
                    error: err.to_string(),
                },
            )?);

        let swap_buy_quote_amount: AmountType =
            f64::from_le_bytes(self.inner_buffer[375..383].try_into().map_err(
                |err: TryFromSliceError| SerializerError::DecodeError {
                    msg: format!(
                        "Invalid buy quite amount {:?}",
                        bytes_to_decimal_string(&self.inner_buffer[375..383])
                    ),
                    error: err.to_string(),
                },
            )?);

        let swap_sell_quote_amount: AmountType =
            f64::from_le_bytes(self.inner_buffer[383..391].try_into().map_err(
                |err: TryFromSliceError| SerializerError::DecodeError {
                    msg: format!(
                        "Invalid sell quote amount {:?}",
                        bytes_to_decimal_string(&self.inner_buffer[383..391])
                    ),
                    error: err.to_string(),
                },
            )?);

        let arbitrage = ArbitrageMessage {
            sequence_id,
            slot,
            buy_side_info: ArbitrageSide {
                lp_name: buy_lp_name.to_string(),
                lp_address: buy_address,
                base_qty: buy_base_qty,
                quote_amount: buy_quote_amount,
                price: buy_price,
                fee: buy_fee,
                lp_last_upd_slot: buy_lp_last_upd_slot,
                lp_last_upd_txn_sig: buy_lp_last_upd_txn_sig,
            },
            sell_side_info: ArbitrageSide {
                lp_name: sell_lp_name.to_string(),
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
        };
        Ok(arbitrage)
    }

    fn deserialize_swap(&mut self, buffer: &[u8]) -> Result<crate::message::SwapMessage> {
        if buffer.len() != self.inner_buffer.len() {
            bail!(SerializerError::InvalidBufferSize {
                size: buffer.len(),
                expected: self.inner_buffer.len(),
            });
        }

        self.inner_buffer.copy_from_slice(buffer);

        let sequence_id = SequenceId::from_bytes(&self.inner_buffer[1..13])?;

        let lp_name = std::str::from_utf8(&self.inner_buffer[13..45])
            .map_err(|err| SerializerError::DecodeError {
                msg: format!(
                    "Invalid LP name {:?}",
                    bytes_to_decimal_string(&self.inner_buffer[13..45])
                ),
                error: err.to_string(),
            })?
            .trim();

        let lp_address = Pubkey::try_from(&self.inner_buffer[45..77]).map_err(|err| {
            SerializerError::DecodeError {
                msg: format!(
                    "Invalid buy address {:?}",
                    bytes_to_decimal_string(&self.inner_buffer[45..77])
                ),
                error: err.to_string(),
            }
        })?;

        let swap_type: SwapType =
            SwapType::try_from(self.inner_buffer[77]).map_err(|err| SerializerError::DecodeError {
                msg: format!("Invalid swap type code {:?}", self.inner_buffer[77]),
                error: err.to_string(),
            })?;

        let base_qty: AmountType = f64::from_le_bytes(self.inner_buffer[78..86].try_into().map_err(
            |err: TryFromSliceError| SerializerError::DecodeError {
                msg: format!(
                    "Invalid base qty {:?}",
                    bytes_to_decimal_string(&self.inner_buffer[78..86])
                ),
                error: err.to_string(),
            },
        )?);

        let quote_amount: AmountType =
            f64::from_le_bytes(self.inner_buffer[86..94].try_into().map_err(
                |err: TryFromSliceError| SerializerError::DecodeError {
                    msg: format!(
                        "Invalid buy quote qty {:?}",
                        bytes_to_decimal_string(&self.inner_buffer[86..94])
                    ),
                    error: err.to_string(),
                },
            )?);

        Ok(SwapMessage {
            sequence_id,
            lp_name: lp_name.to_string(),
            lp_address,
            swap_type,
            base_qty,
            quote_amount,
        })
    }
}
