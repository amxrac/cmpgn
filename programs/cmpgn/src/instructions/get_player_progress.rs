use anchor_lang::prelude::*;

use crate::PlayerProgress;

#[event]
pub struct PlayerProgressEvent {
    pub player: Pubkey,
    pub campaign_id: u8,
    pub completed_bugs: Vec<u8>,
    pub total_completed_bugs: u8,
}

#[derive(Accounts)]
#[instruction(campaign_id: u8)]
pub struct GetPlayerProgress<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(
        mut,
        seeds = [b"progress", campaign_id.to_le_bytes().as_ref(), player.key().as_ref()],
        bump
    )]
    pub player_progress: Account<'info, PlayerProgress>,
}

impl<'info> GetPlayerProgress<'info> {
    pub fn get_player_progress(&mut self, campaign_id: u8) -> Result<()> {
        emit!(PlayerProgressEvent {
            player: self.player.key(),
            campaign_id,
            completed_bugs: self.player_progress.completed_bugs.clone(),
            total_completed_bugs: self.player_progress.total_completed_bugs
        });

        Ok(())
    }
}

pub fn handler(ctx: Context<GetPlayerProgress>, campaign_id: u8) -> Result<()> {
    ctx.accounts.get_player_progress(campaign_id)
}
