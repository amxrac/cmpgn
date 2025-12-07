use anchor_lang::prelude::*;

use crate::PlayerProgress;

#[event]
pub struct CompletedBugEvent {
    pub player: Pubkey,
    pub campaign_id: u8,
    pub bug_id: u8,
    pub completed: bool,
}

#[derive(Accounts)]
#[instruction(campaign_id: u8, bug_id: u8)]
pub struct HasCompletedBug<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(
        mut,
        seeds = [b"progress", campaign_id.to_le_bytes().as_ref(), player.key().as_ref()],
        bump
    )]
    pub player_progress: Account<'info, PlayerProgress>,
}

impl<'info> HasCompletedBug<'info> {
    pub fn has_completed_bug(&mut self, campaign_id: u8, bug_id: u8) -> Result<()> {
        let completed = self.player_progress.completed_bugs.contains(&bug_id);

        emit!(CompletedBugEvent {
            player: self.player.key(),
            campaign_id,
            bug_id,
            completed
        });

        Ok(())
    }
}

pub fn handler(ctx: Context<HasCompletedBug>, campaign_id: u8, bug_id: u8) -> Result<()> {
    ctx.accounts.has_completed_bug(campaign_id, bug_id)
}
