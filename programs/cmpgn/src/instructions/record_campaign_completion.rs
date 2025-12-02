use anchor_lang::prelude::*;

use crate::error::ErrorCode;
use crate::{Campaign, CampaignCompletion, PlayerProgress};

#[derive(Accounts)]
#[instruction(campaign_id: u8, bug_id: u8)]
pub struct RecordCampaignCompletion<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(
        mut,
        seeds = [b"completion", campaign_id.to_le_bytes().as_ref(), player.key().as_ref(), bug_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub campaign_completion: Box<Account<'info, CampaignCompletion>>,

    #[account(
        init_if_needed,
        payer = player,
        space = PlayerProgress::DISCRIMINATOR.len() + PlayerProgress::INIT_SPACE,
        seeds = [b"progress", campaign_id.to_le_bytes().as_ref(), player.key().as_ref()],
        bump
    )]
    pub player_progress: Account<'info, PlayerProgress>,

    #[account(
        mut,
        seeds = [b"campaign", campaign_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub campaign: Box<Account<'info, Campaign>>,
    pub system_program: Program<'info, System>,
}

impl<'info> RecordCampaignCompletion<'info> {
    pub fn record_campaign_completion(&mut self, campaign_id: u8, bug_id: u8) -> Result<()> {
        require!(bug_id >= 1 && bug_id <= 20, ErrorCode::InvalidBugId);

        require!(
            campaign_id == self.campaign.campaign_id,
            ErrorCode::InvalidCampaignId
        );

        require!(
            self.campaign_completion.player == self.player.key(),
            ErrorCode::UnauthorizedPlayer
        );

        require!(
            self.campaign_completion.campaign_start.is_some(),
            ErrorCode::CampaignNotStarted
        );

        require!(
            self.campaign_completion.campaign_end.is_none(),
            ErrorCode::CampaignAlreadyCompleted
        );

        let now = Clock::get()?.unix_timestamp;

        if self.player_progress.player == Pubkey::default() {
            self.player_progress.set_inner(PlayerProgress {
                player: self.player.key(),
                campaign_id: campaign_id,
                completed_bugs: Vec::new(),
                total_completed_bugs: 0,
                bump: self.player_progress.bump,
            });
        }

        self.campaign_completion.campaign_end = Some(now);
        self.campaign_completion.timestamp = Some(now);

        if !self.player_progress.completed_bugs.contains(&bug_id) {
            self.player_progress.completed_bugs.push(bug_id);
            self.player_progress.total_completed_bugs += 1;

            self.campaign.total_completions += 1;
        }

        Ok(())
    }
}

pub fn handler(ctx: Context<RecordCampaignCompletion>, campaign_id: u8, bug_id: u8) -> Result<()> {
    ctx.accounts.record_campaign_completion(campaign_id, bug_id)
}
