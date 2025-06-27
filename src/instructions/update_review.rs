use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    // program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    // system_instruction,
};

use crate::state::{AccountState, ReviewError};
use borsh::{BorshDeserialize, BorshSerialize};

pub fn update_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _title: String,
    rating: u8,
    description: String,
) -> ProgramResult {
    msg!("updating review");
    let accounts_info_iter = &mut accounts.iter();

    let updater = next_account_info(accounts_info_iter)?;
    let pda_account = next_account_info(accounts_info_iter)?;

    // SYSTEM PROGRAM IS NOT NEEDED AGAIN AS THE ACCOUNT HAS BEEN INITAILIZED, BUT IT WOULD BE A GOOD PRACTICE TO VERIFY THE OWNER OF THE ACCOUNT
    if pda_account.owner != program_id{
        return Err(ProgramError::IncorrectProgramId);
    }

    if !updater.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    };
    
    if rating > 10 || rating < 1 {
        return Err(ReviewError::InvalidRating.into());
    };

    msg!("\nunpacking state account");
    let mut data_account = AccountState::try_from_slice(&pda_account.data.borrow()[..])?;

    let (pda, _bump_seed) = Pubkey::find_program_address(
        &[updater.key.as_ref(), data_account.title.as_bytes().as_ref()],
        &program_id,
    );

    if pda != *pda_account.key{
        return Err(ReviewError::InvalidPda.into());
    };

    if !data_account.is_initialized{
        return Err(ReviewError::UninitializedAccount.into());
    }

    msg!("\nReview before update:");
    msg!("Title: {}", data_account.title);
    msg!("Rating: {}", data_account.rating);
    msg!("Description: {}", data_account.description);

    data_account.description = description;
    data_account.rating = rating;

    msg!("\nReview after update:");
    msg!("Title: {}", data_account.title);
    msg!("Rating: {}", data_account.rating);
    msg!("Description: {}", data_account.description);

    msg!("\nserializing the data account\n");

    data_account.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;

    msg!("account serialized");
    
    Ok(())
}
