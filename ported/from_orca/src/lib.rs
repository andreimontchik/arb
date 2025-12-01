mod bn;
pub mod orca_tick_math;
mod u256_math;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PortedFromOrcaError {
    #[error("Failed to downcast the number!")]
    NumberDownCastError,
    #[error("Failed to cast the number!")]
    NumberCastError,
}
