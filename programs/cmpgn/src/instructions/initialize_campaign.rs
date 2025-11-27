use anchor_lang::prelude::*;

use crate::Campaign;

#[derive(Accounts)]
#[instruction(campaign_id: u8)]
pub struct InitializeCampaign<'info> {
    #[account(mut)]
    pub game_authority: Signer<'info>,

    #[account(
        init,
        payer = game_authority,
        seeds = [b"campaign", campaign_id.to_le_bytes().as_ref()],
        bump,
        space = Campaign::DISCRIMINATOR.len() + Campaign::INIT_SPACE,
    )]
    pub campaign: Account<'info, Campaign>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeCampaign<'info> {
    pub fn initialize(&mut self, campaign_id: u8, bumps: &InitializeCampaignBumps) -> Result<()> {
        self.campaign.set_inner(Campaign {
            game_authority: self.game_authority.key(),
            campaign_id: campaign_id,
            total_completions: 0,
            bump: bumps.campaign,
        });

        Ok(())
    }
}

pub fn handler(ctx: Context<InitializeCampaign>, campaign_id: u8) -> Result<()> {
    ctx.accounts.initialize(campaign_id, &ctx.bumps)?;
    Ok(())
}
