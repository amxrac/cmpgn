use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct CampaignCompletion {
    pub player: Pubkey,
    pub campaign_id: u8,
    pub campaign_start: Option<i64>,
    pub campaign_end: Option<i64>,
    pub timestamp: Option<i64>,
    pub bug_id: u8,
    pub nft_mint_address: Option<Pubkey>,
    pub bump: u8,
}
