use anchor_lang::prelude::*;

use crate::Campaign;

#[event]
pub struct CampaignStatsEvent {
    pub campaign_id: u8,
    pub total_completions: u8,
}

#[derive(Accounts)]
#[instruction(campaign_id: u8)]
pub struct GetCampaignStats<'info> {
    pub game_authority: Signer<'info>,

    #[account(
            seeds = [b"campaign", campaign_id.to_le_bytes().as_ref()],
            bump,
            has_one = game_authority
        )]
    pub campaign: Account<'info, Campaign>,
}

impl<'info> GetCampaignStats<'info> {
    pub fn get_campaign_stats(&mut self, campaign_id: u8) -> Result<()> {
        emit!(CampaignStatsEvent {
            campaign_id,
            total_completions: self.campaign.total_completions
        });

        Ok(())
    }
}

pub fn handler(ctx: Context<GetCampaignStats>, campaign_id: u8) -> Result<()> {
    ctx.accounts.get_campaign_stats(campaign_id)
}
