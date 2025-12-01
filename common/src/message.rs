mod arbitrage_message;
mod message_filter;
mod swap_message;

use {
    crate::{CommonError, LiquidityPool, TokenAccount},
    solana_sdk::{clock::UnixTimestamp, pubkey::Pubkey, signature::Signature, slot_history::Slot},
    std::{
        cmp::Ordering,
        fmt,
        str::FromStr,
        time::{SystemTime, UNIX_EPOCH},
    },
};
pub use {
    arbitrage_message::{ArbitrageMessage, ArbitrageSide},
    message_filter::MessageFilter,
    swap_message::{SwapMessage, SwapType},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SequenceId {
    major: u32,
    minor: u64,
}

impl SequenceId {
    pub fn new() -> Self {
        SequenceId {
            major: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs()
                .try_into()
                .expect("Invalid Seconds since EPOCH!"),
            minor: 0,
        }
    }

    pub fn major(&self) -> u32 {
        self.major
    }

    pub fn minor(&self) -> u64 {
        self.minor
    }

    pub fn increment_and_get(&mut self) -> Self {
        self.minor += 1;
        self.clone()
    }

    pub fn to_bytes(&self) -> [u8; 12] {
        let mut bytes = [0; 12];

        // Copy major into bytes[0..4]
        bytes[0..4].copy_from_slice(&self.major.to_be_bytes());

        // Copy minor into bytes[4..12]
        bytes[4..12].copy_from_slice(&self.minor.to_be_bytes());

        bytes
    }

    pub fn from_bytes(buffer: &[u8]) -> Result<Self, CommonError> {
        if buffer.len() != 12 {
            return Err(CommonError::InvalidMessageSource {
                src: format!("{:?}", buffer),
                error: "The buffer length should be 12.".to_string(),
            });
        }

        let major_bytes: [u8; 4] =
            buffer[0..4]
                .try_into()
                .map_err(
                    |error: std::array::TryFromSliceError| CommonError::InvalidMessageSource {
                        src: format!("{:?}", buffer),
                        error: format!("{:?}", error),
                    },
                )?;

        let minor_bytes: [u8; 8] =
            buffer[4..12]
                .try_into()
                .map_err(|error| CommonError::InvalidMessageSource {
                    src: format!("{:?}", buffer),
                    error: format!("{:?}", error),
                })?;

        let major = u32::from_be_bytes(major_bytes);
        let minor = u64::from_be_bytes(minor_bytes);

        Ok(SequenceId { major, minor })
    }

    pub fn is_msg_gap(last_msg_seq: &SequenceId, msg_seq: &SequenceId) -> bool {
        msg_seq.major() != last_msg_seq.major() || msg_seq.minor() == last_msg_seq.minor() + 1
    }
}

impl PartialOrd for SequenceId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SequenceId {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.major.cmp(&other.major) {
            Ordering::Equal => self.minor.cmp(&other.minor),
            other => other,
        }
    }
}

impl fmt::Display for SequenceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.major, self.minor)
    }
}

impl FromStr for SequenceId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();

        if parts.len() != 2 {
            return Err(format!("Invalid format ({s})"));
        }

        let major = parts[0].parse::<u32>().map_err(|e| e.to_string())?;
        let minor = parts[1].parse::<u64>().map_err(|e| e.to_string())?;

        Ok(SequenceId { major, minor })
    }
}

#[derive(Debug, PartialEq)]
pub struct BlockUpdateMessage {
    pub slot: Slot,
    pub block_time: Option<UnixTimestamp>,
    pub block_height: Option<u64>,
}

#[derive(Debug, PartialEq)]
pub struct AccountUpdateMessage {
    pub slot: Slot,
    pub address: Pubkey,
    // TODO: consider replacing vector with preallocated buffered byte array
    pub data: Vec<u8>,
    pub txn_signature: Option<Signature>,
}

#[derive(Debug)]
pub enum Message {
    TokenAccountConfiguration(TokenAccount),
    BlockUpdate(BlockUpdateMessage),
    AccountUpdate(AccountUpdateMessage),
    LiquidityPoolConfiguration(LiquidityPool),
    Arbitrage(ArbitrageMessage),
    Swap(SwapMessage),
}

impl Message {
    pub fn sequence_id(msg: &Message) -> &SequenceId {
        match msg {
            Message::Arbitrage(arb_msg) => &arb_msg.sequence_id,
            Message::Swap(arb_msg) => &arb_msg.sequence_id,
            _ => unimplemented!(),
        }
    }

    pub fn slot(msg: &Message) -> Slot {
        match msg {
            Message::Arbitrage(arb_msg) => arb_msg.slot,
            _ => 0,
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Message::BlockUpdate(a), Message::BlockUpdate(b)) => a == b,
            (Message::AccountUpdate(a), Message::AccountUpdate(b)) => a == b,
            (Message::LiquidityPoolConfiguration(a), Message::LiquidityPoolConfiguration(b)) => a == b,
            (Message::Arbitrage(a), Message::Arbitrage(b)) => a == b,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SequenceId;

    #[test]
    fn test_sequencer_equality() {
        let seq_id1 = SequenceId::new();
        let mut seq_id2: SequenceId = SequenceId::new();
        assert_eq!(seq_id1.minor(), seq_id2.minor());

        seq_id2.major = seq_id1.major();
        assert_eq!(seq_id1, seq_id2);
        assert_eq!(seq_id2, seq_id1);

        seq_id2.minor = seq_id1.minor() + 1;
        assert_ne!(seq_id1, seq_id2);
        assert_ne!(seq_id2, seq_id1);

        seq_id2.minor = seq_id1.minor();
        assert_eq!(seq_id1, seq_id2);
        seq_id2.major = seq_id1.major() + 1;
        assert_ne!(seq_id1, seq_id2);
        assert_ne!(seq_id2, seq_id1);
    }

    #[test]
    fn test_sequencer_comparison() {
        let seq_id1 = SequenceId::new();
        let mut seq_id2 = SequenceId::new();

        // The major is different
        seq_id2.minor = seq_id1.minor();
        seq_id2.major = seq_id1.major() + 1;
        assert!(seq_id2 > seq_id1);
        assert!(seq_id1 < seq_id2);

        // The minor is different
        seq_id2.major = seq_id1.major();
        assert_eq!(seq_id1, seq_id2);
        seq_id2.minor = seq_id1.minor() + 1;
        assert!(seq_id2 > seq_id1);
        assert!(seq_id1 < seq_id2);
    }

    #[test]
    fn test_sequencer_increment() {
        let mut seq_id1 = SequenceId::new();
        assert_eq!(seq_id1.minor(), 0);

        let major = seq_id1.major();

        for i in 1..513 {
            let seq_id2 = seq_id1.increment_and_get();
            assert_eq!(seq_id2.major(), major);
            assert_eq!(seq_id2.minor(), i);
            assert_eq!(seq_id2, seq_id1);
            assert_eq!(seq_id2.minor(), seq_id1.minor());
        }
    }

    #[test]
    fn test_validate_for_msg_gap() {
        let mut msg1 = SequenceId::new();
        let mut msg2 = SequenceId::new();
        msg1.major = msg2.major;
        msg1.minor = msg2.minor;

        assert!(!SequenceId::is_msg_gap(&msg1, &msg2));

        // New sequencing
        msg2.major = msg2.major + 1;
        assert!(SequenceId::is_msg_gap(&msg1, &msg2));

        // The expected +1 increment
        msg1.major = msg2.major;
        assert!(!SequenceId::is_msg_gap(&msg1, &msg2));
        msg2.minor = msg1.minor + 1;
        assert!(SequenceId::is_msg_gap(&msg1, &msg2));
    }
}
