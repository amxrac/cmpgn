use anchor_lang::prelude::*;

use crate::error::ErrorCode;
use crate::{Campaign, CampaignCompletion, PlayerProgress};

#[derive(Accounts)]
#[instruction(campaign_id: u8, bug_id: u8)]
pub struct RecordCampaignCompletion<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(
        init_if_needed,
        payer = player,
        space = CampaignCompletion::DISCRIMINATOR.len() + CampaignCompletion::INIT_SPACE,
        seeds = [b"completion", campaign_id.to_le_bytes().as_ref(), player.key().as_ref(), bug_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub campaign_completion: Account<'info, CampaignCompletion>,

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
    pub campaign: Account<'info, Campaign>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RecordCampaignCompletion>, campaign_id: u8, bug_id: u8) -> Result<()> {
    require!(bug_id >= 1 && bug_id <= 20, ErrorCode::InvalidBugId);

    let now = Clock::get()?.unix_timestamp;

    if ctx.accounts.campaign_completion.campaign_start.is_none() {
        ctx.accounts.campaign_completion.player = ctx.accounts.player.key();
        ctx.accounts.campaign_completion.campaign_id = campaign_id;
        ctx.accounts.campaign_completion.campaign_start = Some(now);
        ctx.accounts.campaign_completion.bug_id = bug_id;
        ctx.accounts.campaign_completion.bump = ctx.bumps.campaign_completion;
    }

    if ctx.accounts.player_progress.player == Pubkey::default() {
        ctx.accounts.player_progress.set_inner(PlayerProgress {
            player: ctx.accounts.player.key(),
            campaign_id: ctx.accounts.campaign.campaign_id,
            completed_bugs: Vec::new(),
            total_completed_bugs: 0,
            bump: ctx.bumps.player_progress,
        });
    }

    ctx.accounts.campaign_completion.campaign_end = Some(now);
    ctx.accounts.campaign_completion.timestamp = Some(now);

    if !ctx
        .accounts
        .player_progress
        .completed_bugs
        .contains(&bug_id)
    {
        ctx.accounts.player_progress.completed_bugs.push(bug_id);
        ctx.accounts.player_progress.total_completed_bugs += 1;

        ctx.accounts.campaign.total_completions += 1;
    }

    Ok(())
}
