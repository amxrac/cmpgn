use anchor_lang::prelude::*;

use crate::error::ErrorCode;
use crate::{Campaign, CampaignCompletion};

#[derive(Accounts)]
#[instruction(campaign_id: u8, bug_id: u8)]
pub struct StartCampaign<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(
        init,
        payer = player,
        space = CampaignCompletion::DISCRIMINATOR.len() + CampaignCompletion::INIT_SPACE,
        seeds = [b"completion", campaign_id.to_le_bytes().as_ref(), player.key().as_ref(), bug_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub campaign_completion: Account<'info, CampaignCompletion>,

    // #[account(
    //     mut,
    //     seeds = [b"campaign", campaign_id.to_le_bytes().as_ref()],
    //     bump,
    // )]
    // pub campaign: Box<Account<'info, Campaign>>,
    pub system_program: Program<'info, System>,
}

impl<'info> StartCampaign<'info> {
    pub fn start_campaign(&mut self, campaign_id: u8, bug_id: u8) -> Result<()> {
        require!(bug_id >= 1 && bug_id <= 20, ErrorCode::InvalidBugId);

        let now = Clock::get()?.unix_timestamp;

        self.campaign_completion.set_inner(CampaignCompletion {
            player: self.player.key(),
            campaign_id,
            campaign_start: Some(now),
            campaign_end: None,
            timestamp: None,
            bug_id,
            nft_mint_address: None,
            bump: self.campaign_completion.bump,
        });

        Ok(())
    }
}

pub fn handler(ctx: Context<StartCampaign>, campaign_id: u8, bug_id: u8) -> Result<()> {
    ctx.accounts.start_campaign(campaign_id, bug_id)?;

    Ok(())
}
