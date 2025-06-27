use solana_program::{
    account_info:: AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::instructions::unpack::ReviewIx;
use crate::instructions::add_review::add_review;
use crate::instructions::update_review::update_review;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    ix_data: &[u8],
) -> ProgramResult {
    let ix = ReviewIx::unpack(&ix_data)?;

    match ix {
        ReviewIx::AddReview(review_payload) => {
            return add_review(
                program_id, 
                accounts, 
                review_payload.title, 
                review_payload.rating, 
                review_payload.description
            );
        },
        ReviewIx::UpdateReview(review_payload) => {
            return update_review(
                program_id, 
                accounts, 
                review_payload.title, 
                review_payload.rating, 
                review_payload.description
            );
        },
    }
}
