pub use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReviewError {
    #[error("Account not initialized yet")]
    UninitializedAccount,

    #[error("Pda account not matching Pda passed in")]
    InvalidPda,

    #[error("Rating greater than 10 or less than 1")]
    InvalidRating,
}
