pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("AuXF95nT7WS865AzQpuj3os9r6DjTYY9ekh4mGgG6gfL");

#[program]
pub mod cmpgn {
    use super::*;

    pub fn initialize(ctx: Context<InitializeCampaign>, campaign_id: u8) -> Result<()> {
        instructions::initialize_campaign::handler(ctx, campaign_id)
    }

    pub fn create_collection(
        ctx: Context<CreateCollection>,
        campaign_id: u8,
        args: CreateCollectionArgs,
    ) -> Result<()> {
        instructions::create_collection::handler(ctx, campaign_id, args)
    }

    pub fn start_campaign(ctx: Context<StartCampaign>, campaign_id: u8, bug_id: u8) -> Result<()> {
        instructions::start_campaign::handler(ctx, campaign_id, bug_id)
    }

    pub fn record_campaign_completion(
        ctx: Context<RecordCampaignCompletion>,
        campaign_id: u8,
        bug_id: u8,
    ) -> Result<()> {
        instructions::record_campaign_completion::handler(ctx, campaign_id, bug_id)
    }

    pub fn mint_nft(
        ctx: Context<MintNft>,
        bug_id: u8,
        name: String,
        nft_uri: String,
    ) -> Result<()> {
        instructions::mint_nft::handler(ctx, bug_id, name, nft_uri)
    }
}
