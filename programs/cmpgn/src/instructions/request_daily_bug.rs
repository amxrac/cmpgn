use crate::error::ErrorCode;
use crate::DailyBug;
use anchor_lang::prelude::*;

use orao_solana_vrf::cpi::accounts::RequestV2;
use orao_solana_vrf::program::OraoVrf;
use orao_solana_vrf::state::NetworkState;
use orao_solana_vrf::CONFIG_ACCOUNT_SEED;
use orao_solana_vrf::RANDOMNESS_ACCOUNT_SEED;

#[derive(Accounts)]
#[instruction(force: [u8; 32])]
pub struct RequestDailyBug<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(
        init_if_needed,
        payer = player,
        space = DailyBug::DISCRIMINATOR.len() + DailyBug::INIT_SPACE,
        seeds = [b"daily_bug_seed"],
        bump,
    )]
    pub bug_state: Account<'info, DailyBug>,

    /// CHECK:
    #[account(
        mut,
        seeds = [RANDOMNESS_ACCOUNT_SEED, &force],
        bump,
        seeds::program = orao_solana_vrf::ID
    )]
    pub random: UncheckedAccount<'info>,

    /// CHECK:
    #[account(mut)]
    pub treasury: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [CONFIG_ACCOUNT_SEED],
        bump,
        seeds::program = orao_solana_vrf::ID
    )]
    pub config: Account<'info, NetworkState>,
    pub vrf: Program<'info, OraoVrf>,
    pub system_program: Program<'info, System>,
}

impl<'info> RequestDailyBug<'info> {
    pub fn request_daily_bug(&mut self, force: [u8; 32]) -> Result<()> {
        let current_day = Clock::get()?.unix_timestamp / 86400;

        require!(
            self.bug_state.day < current_day || self.bug_state.force == [0u8; 32],
            ErrorCode::BugAlreadyRequested
        );

        self.bug_state.day = current_day;
        self.bug_state.bug_id = None;
        self.bug_state.force = force;

        let cpi_prog = self.vrf.to_account_info();
        let cpi_accounts = RequestV2 {
            payer: self.player.to_account_info(),
            network_state: self.config.to_account_info(),
            treasury: self.treasury.to_account_info(),
            request: self.random.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_prog, cpi_accounts);
        orao_solana_vrf::cpi::request_v2(cpi_ctx, force)?;

        Ok(())
    }
}

pub fn handler(ctx: Context<RequestDailyBug>, force: [u8; 32]) -> Result<()> {
    ctx.accounts.request_daily_bug(force)
}
