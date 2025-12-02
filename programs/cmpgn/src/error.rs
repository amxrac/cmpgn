use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Campaign Id")]
    InvalidCampaignId,
    #[msg("Campaign not initialized")]
    CampaignNotStarted,
    #[msg("Campaign already completed")]
    CampaignAlreadyCompleted,
    #[msg("Invalid Bug Id")]
    InvalidBugId,
    #[msg("The payer is not the program's upgrade authority.")]
    NotAuthorized,
    #[msg("The collection has already been initialized.")]
    CollectionAlreadyInitialized,
    #[msg("The asset has already been initialized.")]
    AssetAlreadyInitialized,
    #[msg("The collection is not initialized.")]
    CollectionNotInitialized,
    #[msg("The collection is invalid.")]
    InvalidCollection,
    #[msg("Unauthorized Player.")]
    UnauthorizedPlayer,
}
