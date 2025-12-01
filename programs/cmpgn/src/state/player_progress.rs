use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PlayerProgress {
    pub player: Pubkey,
    pub campaign_id: u8,
    #[max_len(20)]
    pub completed_bugs: Vec<u8>,
    pub total_completed_bugs: u8,
    pub bump: u8,
}
