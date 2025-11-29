use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Campaign Id")]
    InvalidCampaignId,
    #[msg("Invalid Bug Id")]
    BugIdOutOfRange,
    #[msg("The payer is not the program's upgrade authority.")]
    NotAuthorized,
    #[msg("The collection has already been initialized.")]
    CollectionAlreadyInitialized,
}
