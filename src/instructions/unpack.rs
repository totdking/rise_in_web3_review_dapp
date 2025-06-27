use solana_program::program_error::ProgramError;

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

pub enum ReviewIx {
    AddReview(ReviewPayload),
    UpdateReview(ReviewPayload),
}

#[derive(Debug, BorshSerialize, BorshDeserialize, BorshSchema)]
pub struct ReviewPayload {
    pub title: String,
    pub rating: u8,
    pub description: String,
}

impl ReviewIx {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        // THE try_from_slice() deserializes the remaining data gotten from the split_first method into the ReviewPayload struct.
        let payload = ReviewPayload::try_from_slice(rest)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        let unpacked = match variant {
            0 => Self::AddReview(payload),
            1 => Self::UpdateReview(payload),
            _ => return Err(ProgramError::InvalidInstructionData),
        };
        Ok(unpacked)
    }
}
