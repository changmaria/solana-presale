use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum CustomError {
    #[error("invalid instruction")]
    InvalidInstruction,
    #[error("Min buy limit not reached")]
    MinLimitNotReached,
    #[error("Max buy limit reached in this presale")]
    MaxLimitReached,
}

impl From<CustomError> for ProgramError {
    fn from(e: CustomError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
