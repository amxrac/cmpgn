use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Campaign Id")]
    InvalidCampaignId,
    #[msg("Invalid Bug Id")]
    BugIdOutOfRange,
}
