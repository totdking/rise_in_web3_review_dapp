use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};

use crate::state::{AccountState, ReviewError};
use borsh::{BorshDeserialize, BorshSerialize};
use thiserror::Error;

pub fn add_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    rating: u8,
    description: String,
) -> ProgramResult {
    msg!("Adding  review...");
    msg!("Title: {}", title);
    msg!("Rating: {}", rating);
    msg!("Description: {}", description);

    let accounts_info_iter = &mut accounts.iter();

    let initializer = next_account_info(accounts_info_iter)?;
    let pda_account = next_account_info(accounts_info_iter)?;
    let system_program = next_account_info(accounts_info_iter)?;

    let system_program_hardcoded = Pubkey::from_str_const("11111111111111111111111111111111");

    if !initializer.is_signer {
        msg!("\nSigner check passed");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Added this check to be sure that only the given solana program is the needed owner
    if *system_program.key != system_program_hardcoded {
        return Err(ProgramError::IllegalOwner);
    }

    let (pda, bump_seed) = Pubkey::find_program_address(
        &[initializer.key.as_ref(), title.as_bytes().as_ref()],
        program_id,
    );

    if pda != *pda_account.key {
        msg!("\nPDA check passed");
        return Err(ProgramError::InvalidArgument);
    }

    if rating > 10 || rating < 1 {
        return Err(ReviewError::InvalidRating.into());
    }

    let account_len: usize = 1000;

    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    #[derive(Debug, Error)]
    pub enum Error {
        #[error("Invalid create account space")]
        IncorrectSpaceLen,
    }

    let ix = system_instruction::create_account(
        initializer.key,
        pda_account.key,
        rent_lamports,
        account_len
            .try_into()
            .map_err(|_| ProgramError::Custom(Error::IncorrectSpaceLen as u32))?,
        program_id,
    );

    let input_accounts = [
        initializer.clone(),
        pda_account.clone(),
        system_program.clone(),
    ];

    invoke_signed(
        &ix,
        &input_accounts,
        &[&[
            initializer.key.as_ref(),
            title.as_bytes().as_ref(),
            &[bump_seed],
        ]],
    )?;
    msg!("pda created");

    //TRY FROM SLICE IS A DESERIALIZING METHOD FROM THE BORSHDERIALIZE TO GET STRUCT DATA FROM A &[u8] DATA SERIALIZED BY A SERIALIZED/ SPLIT_FIRST METHOD
    // IN THE CODE THEY USED
    // ```let mut account_data = try_from_slice_unchecked::<AccountState>(&pda_account.data.borrow()).unwrap();```
    let mut data_account = AccountState::try_from_slice(&pda_account.data.borrow())?;
    msg!("Borrowed account data\n");

    msg!("Checking if account is initialized");
    if !data_account.is_initialized {
        return Err(ProgramError::AccountAlreadyInitialized);
    }
    msg!("\nAccount is not pre initialized");

    data_account.description = description;
    data_account.title = title;
    data_account.rating = rating;
    data_account.is_initialized = true;

    msg!("\nserializing account");
    data_account.serialize(&mut &mut initializer.data.borrow_mut()[..])?;

    msg!("\nstate account serialized");
    Ok(())
}
