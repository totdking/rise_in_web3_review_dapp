pub use crate::errors::ReviewError;
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::program_error::ProgramError;
use solana_program::program_pack::{IsInitialized, Sealed};

#[derive(Debug, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct AccountState {
    pub is_initialized: bool,
    pub rating: u8,
    pub description: String,
    pub title: String,
}

impl Sealed for AccountState {}

impl IsInitialized for AccountState {
    fn is_initialized(&self) -> bool {
        return self.is_initialized;
    }
}

impl From<ReviewError> for ProgramError {
    fn from(e: ReviewError) -> Self {
        return ProgramError::Custom(e as u32);
    }
}
