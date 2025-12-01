/// The file is ported from the [Orca Whirlpools](https://github.com/orca-so/whirlpools/blob/main/programs/whirlpool/src/math/bn.rs repo on 03/01/2024.
/// The Orca Whirlpools source code is failing to compile because it uses incomptatible version of the Borsh serializer.
use {
    crate::PortedFromOrcaError,
    std::{borrow::BorrowMut, convert::TryInto, mem::size_of},
    uint::construct_uint,
};

construct_uint! {
    // U256 of [u64; 4]
    pub struct U256(4);
}

impl U256 {
    pub fn try_into_u64(self) -> Result<u64, PortedFromOrcaError> {
        self.try_into()
            .map_err(|_| PortedFromOrcaError::NumberCastError)
    }

    pub fn try_into_u128(self) -> Result<u128, PortedFromOrcaError> {
        self.try_into()
            .map_err(|_| PortedFromOrcaError::NumberCastError)
    }

    pub fn from_le_bytes(bytes: [u8; 32]) -> Self {
        U256::from_little_endian(&bytes)
    }

    pub fn to_le_bytes(self) -> [u8; 32] {
        let mut buf: Vec<u8> = Vec::with_capacity(size_of::<Self>());
        self.to_little_endian(buf.borrow_mut());

        let mut bytes: [u8; 32] = [0u8; 32];
        bytes.copy_from_slice(buf.as_slice());
        bytes
    }
}

#[cfg(test)]
mod test_u256 {
    use super::*;

    #[test]
    fn test_into_u128_ok() {
        let a = U256::from(2653u128);
        let b = U256::from(1232u128);
        let sum = a + b;
        let d: u128 = sum.try_into_u128().unwrap();
        assert_eq!(d, 3885u128);
    }

    #[test]
    fn test_into_u128_error() {
        let a = U256::from(u128::MAX);
        let b = U256::from(u128::MAX);
        let sum = a + b;
        let c: Result<u128, PortedFromOrcaError> = sum.try_into_u128();
        assert_eq!(c.is_err(), true);
    }

    #[test]
    fn test_as_u128_ok() {
        let a = U256::from(2653u128);
        let b = U256::from(1232u128);
        let sum = a + b;
        let d: u128 = sum.as_u128();
        assert_eq!(d, 3885u128);
    }

    #[test]
    #[should_panic(expected = "Integer overflow when casting to u128")]
    fn test_as_u128_panic() {
        let a = U256::from(u128::MAX);
        let b = U256::from(u128::MAX);
        let sum = a + b;
        let _: u128 = sum.as_u128();
    }

    #[test]
    fn test_into_u64_ok() {
        let a = U256::from(2653u64);
        let b = U256::from(1232u64);
        let sum = a + b;
        let d: u64 = sum.try_into_u64().unwrap();
        assert_eq!(d, 3885u64);
    }

    #[test]
    fn test_into_u64_error() {
        let a = U256::from(u64::MAX);
        let b = U256::from(u64::MAX);
        let sum = a + b;
        let c: Result<u64, PortedFromOrcaError> = sum.try_into_u64();
        assert_eq!(c.is_err(), true);
    }

    #[test]
    fn test_as_u64_ok() {
        let a = U256::from(2653u64);
        let b = U256::from(1232u64);
        let sum = a + b;
        let d: u64 = sum.as_u64();
        assert_eq!(d, 3885u64);
    }

    #[test]
    #[should_panic(expected = "Integer overflow when casting to u64")]
    fn test_as_u64_panic() {
        let a = U256::from(u64::MAX);
        let b = U256::from(u64::MAX);
        let sum = a + b;
        let _: u64 = sum.as_u64(); // panic overflow
    }
}
