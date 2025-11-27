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
}
