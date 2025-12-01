mod binary_serializer;
mod csv_serializer;

use {
    crate::{
        message::{AccountUpdateMessage, ArbitrageMessage, BlockUpdateMessage, Message, SwapMessage},
        LiquidityPool,
    },
    anyhow::{bail, Result},
    std::fmt,
    thiserror::Error,
};
pub use {binary_serializer::BinarySerializer, csv_serializer::CsvSerializer};

#[derive(Error, Debug)]
pub enum SerializerError {
    #[error("{error}")]
    InvalidBuffer { error: String },
    #[error("{size}|{expected}")]
    InvalidBufferSize { size: usize, expected: usize },
    #[error("{size}|{expected}")]
    SliceTooShort { size: usize, expected: usize },
    #[error("{msg}")]
    InvalidMessageFormat { msg: String },
    #[error("{msg_type}")]
    InvalidMessageType { msg_type: String },
    #[error("({msg}), {error}")]
    ParseError { msg: String, error: String },
    #[error("({msg}), {error}")]
    DecodeError { msg: String, error: String },
}

pub const BUFFER_SIZE: usize = 1024;
pub const ZERO_BYTE: u8 = 0;

const EMPTY_STRING: &str = "";
const COMMA_CHAR: char = ',';
const SPACE_CHAR_CODE: u8 = 32;
const COMMA_CHAR_CODE: u8 = 44;

#[derive(PartialEq, Debug)]
pub enum MessageType {
    TokenAccountConfiguration = 1,
    BlockUpdate = 10,
    AccountUpdate = 20,
    LiquidityPoolConfiguration = 30,
    Arbitrage = 40,
    Swap = 50,
}

impl From<&Message> for MessageType {
    fn from(msg: &Message) -> Self {
        match msg {
            Message::TokenAccountConfiguration(..) => MessageType::TokenAccountConfiguration,
            Message::LiquidityPoolConfiguration { .. } => MessageType::LiquidityPoolConfiguration,
            Message::BlockUpdate { .. } => MessageType::BlockUpdate,
            Message::AccountUpdate { .. } => MessageType::AccountUpdate,
            Message::Arbitrage { .. } => MessageType::Arbitrage,
            Message::Swap(..) => MessageType::Swap,
        }
    }
}

impl MessageType {
    pub fn to_string(&self) -> String {
        match self {
            MessageType::TokenAccountConfiguration => "TokenAccountConfiguration".to_string(),
            MessageType::BlockUpdate => "BlockUpdate".to_string(),
            MessageType::AccountUpdate => "AccountUpdate".to_string(),
            MessageType::LiquidityPoolConfiguration => "LiquidityPoolConfiguration".to_string(),
            MessageType::Arbitrage => "Arbitrage".to_string(),
            MessageType::Swap => "Swap".to_string(),
        }
    }
}

impl TryFrom<&str> for MessageType {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "BlockUpdate" => Ok(MessageType::BlockUpdate),
            "AccountUpdate" => Ok(MessageType::AccountUpdate),
            "LiquidityPoolConfiguration" => Ok(MessageType::LiquidityPoolConfiguration),
            "Arbitrage" => Ok(MessageType::Arbitrage),
            "Swap" => Ok(MessageType::Swap),
            _ => bail!(SerializerError::InvalidMessageType {
                msg_type: value.to_string(),
            }),
        }
    }
}

impl TryFrom<u8> for MessageType {
    type Error = anyhow::Error;

    fn try_from(byte: u8) -> Result<Self> {
        match byte {
            10 => Ok(MessageType::BlockUpdate),
            20 => Ok(MessageType::AccountUpdate),
            30 => Ok(MessageType::LiquidityPoolConfiguration),
            40 => Ok(MessageType::Arbitrage),
            50 => Ok(MessageType::Swap),
            _ => bail!(SerializerError::InvalidMessageType {
                msg_type: format!("{}", byte),
            }),
        }
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait Serializer {
    fn new() -> Self;

    fn serialize_message(&mut self, msg: &Message, buffer: &mut Vec<u8>) -> Result<usize> {
        match msg {
            Message::LiquidityPoolConfiguration(msg) => self.serialize_liquidity_pool(msg, buffer),
            Message::BlockUpdate(msg) => self.serialize_block_update(msg, buffer),
            Message::AccountUpdate(msg) => self.serialize_account_update(msg, buffer),
            Message::Arbitrage(msg) => self.serialize_arbitrage(msg, buffer),
            Message::Swap(msg) => self.serialize_swap(msg, buffer),
            _ => unimplemented!(),
        }
    }

    fn serialize_liquidity_pool(&mut self, _lp: &LiquidityPool, _buffer: &mut Vec<u8>) -> Result<usize> {
        unimplemented!();
    }

    fn serialize_block_update(
        &mut self,
        _msg: &BlockUpdateMessage,
        _buffer: &mut Vec<u8>,
    ) -> Result<usize> {
        unimplemented!();
    }

    fn serialize_account_update(
        &mut self,
        _msg: &AccountUpdateMessage,
        _buffer: &mut Vec<u8>,
    ) -> Result<usize> {
        unimplemented!();
    }

    fn serialize_arbitrage(&mut self, _msg: &ArbitrageMessage, _buffer: &mut Vec<u8>) -> Result<usize> {
        unimplemented!();
    }

    fn serialize_swap(&mut self, _msg: &SwapMessage, _buffer: &mut Vec<u8>) -> Result<usize> {
        unimplemented!();
    }

    fn deserialize_message_type(&self, _buffer: &[u8]) -> Result<MessageType>;

    fn deserialize_message(&mut self, buffer: &[u8]) -> Result<Message> {
        match self.deserialize_message_type(buffer)? {
            MessageType::BlockUpdate => Ok(Message::BlockUpdate(self.deserialize_block_update(buffer)?)),
            MessageType::AccountUpdate => {
                Ok(Message::AccountUpdate(self.deserialize_account_update(buffer)?))
            }
            MessageType::LiquidityPoolConfiguration => Ok(Message::LiquidityPoolConfiguration(
                self.deserialize_account_configuration(buffer)?,
            )),
            MessageType::Arbitrage => Ok(Message::Arbitrage(self.deserialize_arbitrage(buffer)?)),
            MessageType::Swap => Ok(Message::Swap(self.deserialize_swap(buffer)?)),
            _ => unimplemented!(),
        }
    }

    fn deserialize_account_configuration(&self, _buffer: &[u8]) -> Result<LiquidityPool> {
        unimplemented!();
    }

    fn deserialize_block_update(&mut self, _buffer: &[u8]) -> Result<BlockUpdateMessage> {
        unimplemented!();
    }

    fn deserialize_account_update(&mut self, _buffer: &[u8]) -> Result<AccountUpdateMessage> {
        unimplemented!();
    }

    fn deserialize_arbitrage(&mut self, _buffer: &[u8]) -> Result<ArbitrageMessage> {
        unimplemented!();
    }

    fn deserialize_swap(&mut self, _buffer: &[u8]) -> Result<SwapMessage> {
        unimplemented!();
    }
}

#[inline]
fn check_buffer_size(buffer: &mut Vec<u8>, size: usize) {
    if buffer.len() < size {
        buffer.resize(size, 0);
    }
}

#[inline]
fn _write_to_buffer(src: &[u8], buffer: &mut Vec<u8>, offset: usize) -> usize {
    let src_size = src.len();
    check_buffer_size(buffer, offset + src_size);
    buffer[offset..src_size + offset].copy_from_slice(src);
    offset + src_size
}

#[inline]
fn _append_to_buffer(byte: u8, buffer: &mut Vec<u8>, offset: usize) -> usize {
    check_buffer_size(buffer, offset + 1);
    buffer[offset] = byte;
    offset + 1
}

#[inline]
pub fn write_str_to_buffer(msg: &str, buffer: &mut Vec<u8>, offset: usize) -> usize {
    let msg_bt = msg.as_bytes();
    let msg_len = msg_bt.len();
    check_buffer_size(buffer, msg_len + offset);
    buffer[offset..offset + msg_len].copy_from_slice(msg_bt);
    offset + msg_len
}

#[inline]
pub fn try_write_str_to_slice(src: &str, slice: &mut [u8]) -> Result<usize> {
    let src_bytes = src.as_bytes();
    let src_len = src_bytes.len();

    if slice.len() < src_len {
        bail!(SerializerError::SliceTooShort {
            size: slice.len(),
            expected: src_len,
        });
    }

    slice[..src_len].copy_from_slice(src_bytes);
    if slice.len() > src_len {
        slice[src_len..].fill(SPACE_CHAR_CODE);
    }

    Ok(src_len)
}

#[inline]
fn bytes_to_decimal_string(bytes: &[u8]) -> String {
    let result = bytes
        .iter()
        .map(|&byte| byte.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    format!("[{result}]")
}

#[cfg(test)]
mod tests {
    use {
        super::{
            MessageType, _append_to_buffer, _write_to_buffer, bytes_to_decimal_string,
            try_write_str_to_slice,
        },
        crate::serializer::{check_buffer_size, write_str_to_buffer, SPACE_CHAR_CODE},
    };

    #[test]
    fn test_check_buffer_size() {
        let init_size = 8;
        let mut buffer: Vec<u8> = vec![0; init_size];
        assert_eq!(buffer.len(), init_size);

        // Resize
        for i in 0..16 {
            check_buffer_size(&mut buffer, i);
            if i <= init_size {
                assert_eq!(buffer.len(), init_size);
            } else {
                assert_eq!(buffer.len(), i);
            }
        }

        assert_eq!(buffer.len(), 15);
    }

    #[test]
    fn test_write_to_buffer() {
        let src = vec![1u8, 1, 2, 2, 3, 3];

        let mut buffer = Vec::<u8>::new();
        _write_to_buffer(&src, &mut buffer, 0);
        assert_eq!(src, buffer);

        for i in 1..100 {
            _write_to_buffer(&src, &mut buffer, i);
        }
        assert_eq!(buffer.len(), 105);

        for i in 0..101 {
            assert_eq!(buffer[i], 1);
        }
        assert_eq!(buffer[101], 2);
        assert_eq!(buffer[102], 2);
        assert_eq!(buffer[103], 3);
        assert_eq!(buffer[104], 3);
    }

    #[test]
    fn test_append_to_buffer() {
        let mut buffer = Vec::<u8>::new();
        for i in 0..100 {
            _append_to_buffer(i as u8, &mut buffer, i);
        }
        for i in 0..100 {
            assert_eq!(buffer[i], i as u8);
        }
    }

    #[test]
    fn test_write_str_to_buffer() {
        const MESSAGE1: &str = "Test Message 1";
        const MESSAGE2: &str = "Test Message 2";
        const MESSAGE3: &str = "Test Message 3";
        let mut buffer = Vec::<u8>::new();

        let mut size = write_str_to_buffer(MESSAGE1, &mut buffer, 0);
        assert_eq!(buffer, MESSAGE1.as_bytes());
        assert_eq!(size, MESSAGE1.len());

        size = write_str_to_buffer(MESSAGE2, &mut buffer, MESSAGE1.len());
        assert_eq!(buffer, format!("{}{}", MESSAGE1, MESSAGE2).as_bytes());
        assert_eq!(size, MESSAGE1.len() + MESSAGE2.len());

        size = write_str_to_buffer(MESSAGE3, &mut buffer, MESSAGE1.len() + MESSAGE2.len());
        assert_eq!(buffer, format!("{}{}{}", MESSAGE1, MESSAGE2, MESSAGE3).as_bytes());
        assert_eq!(size, MESSAGE1.len() + MESSAGE2.len() + MESSAGE3.len());
    }

    #[test]
    fn test_message_type_parsing() {
        assert_eq!(
            MessageType::BlockUpdate,
            MessageType::try_from(MessageType::BlockUpdate.to_string().as_str()).unwrap()
        );
        assert_eq!(
            MessageType::AccountUpdate,
            MessageType::try_from(MessageType::AccountUpdate.to_string().as_str()).unwrap()
        );
        assert_eq!(
            MessageType::LiquidityPoolConfiguration,
            MessageType::try_from(MessageType::LiquidityPoolConfiguration.to_string().as_str()).unwrap()
        );
        assert_eq!(
            MessageType::Arbitrage,
            MessageType::try_from(MessageType::Arbitrage.to_string().as_str()).unwrap()
        );
        assert_eq!(
            MessageType::Swap,
            MessageType::try_from(MessageType::Swap.to_string().as_str()).unwrap()
        );

        // Invalid message type
        assert!(MessageType::try_from("Dummy").is_err());
    }

    #[test]
    fn test_try_write_str_to_slice() {
        let src = "012345678";
        let src_bytes = src.as_bytes();
        let src_len = src_bytes.len();
        let mut buffer: Vec<u8> = vec![SPACE_CHAR_CODE; 32];

        // Slice is too short.
        assert!(try_write_str_to_slice(src, &mut buffer[0..8]).is_err());

        // Exact match.
        buffer.fill(SPACE_CHAR_CODE);
        assert_eq!(
            try_write_str_to_slice(src, &mut buffer[0..src_len]).unwrap(),
            src_len
        );
        assert_eq!(src_bytes[..], buffer[0..src_len]);
        assert_eq!(src, String::from_utf8(buffer[0..src_len].to_vec()).unwrap());

        // Slice is longer that the source string.
        buffer.fill(SPACE_CHAR_CODE);
        assert_eq!(
            try_write_str_to_slice(src, &mut buffer[0..src_len + 6]).unwrap(),
            src_len
        );
        assert_eq!(src_bytes[..], buffer[0..src_len]);
        assert_eq!(src, String::from_utf8(buffer[0..src_len].to_vec()).unwrap());
        assert!(!&buffer[src_len..].iter().any(|&byte| byte != SPACE_CHAR_CODE));
    }

    #[test]
    fn test_bytes_to_decimal_string() {
        let bytes = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(bytes_to_decimal_string(&bytes), "[1, 2, 3, 4, 5, 6, 7, 8, 9]");
    }
}
