use {
    super::SequenceId,
    crate::{AmountType, CommonError},
    anyhow::{bail, Result},
    solana_sdk::pubkey::Pubkey,
};

#[derive(Debug, PartialEq)]
pub struct SwapMessage {
    pub sequence_id: SequenceId,
    pub lp_name: String,
    pub lp_address: Pubkey,
    pub swap_type: SwapType,
    pub base_qty: AmountType,
    pub quote_amount: AmountType,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SwapType {
    Buy = 1,
    Sell = 2,
}

impl TryFrom<&str> for SwapType {
    type Error = anyhow::Error;

    fn try_from(code: &str) -> Result<Self> {
        match code {
            "Buy" => Ok(SwapType::Buy),
            "Sell" => Ok(SwapType::Sell),
            _ => bail!(CommonError::InvalidSwapType {
                code: code.to_string(),
            }),
        }
    }
}

impl TryFrom<u8> for SwapType {
    type Error = anyhow::Error;

    fn try_from(code: u8) -> Result<Self> {
        match code {
            1 => Ok(SwapType::Buy),
            2 => Ok(SwapType::Sell),
            _ => bail!(CommonError::InvalidSwapType {
                code: code.to_string(),
            }),
        }
    }
}

impl ToString for SwapType {
    fn to_string(&self) -> String {
        match self {
            SwapType::Buy => "Buy".to_string(),
            SwapType::Sell => "Sell".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::message::swap_message::SwapType;

    #[test]
    fn test_swap_type_to_string_convertions() {
        assert_eq!(
            SwapType::try_from(SwapType::Buy.to_string().as_str()).unwrap(),
            SwapType::Buy
        );
        assert_eq!(
            SwapType::try_from(SwapType::Sell.to_string().as_str()).unwrap(),
            SwapType::Sell
        );
        assert!(SwapType::try_from("Dummy").is_err());
    }
}
