use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Campaign {
    pub game_authority: Pubkey,
    pub campaign_id: u8,
    pub total_completions: u8,
    pub bump: u8,
}
