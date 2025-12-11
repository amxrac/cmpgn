use anchor_lang::prelude::*;

// use orao_solana_vrf::program::OraoVrf;
// use orao_solana_vrf::state::NetworkState;
// use orao_solana_vrf::CONFIG_ACCOUNT_SEED;
// use orao_solana_vrf::RANDOMNESS_ACCOUNT_SEED;
// TODO

#[event]
pub struct DailyBugEvent {
    pub bug_id: u8,
}

#[derive(Accounts)]
pub struct GetDailyBug<'info> {
    pub player: Signer<'info>,
}

impl<'info> GetDailyBug<'info> {
    pub fn get_daily_bug(&mut self) -> Result<()> {
        let clock = Clock::get()?;
        let day = clock.unix_timestamp / 86400;
        let bug_id = ((day % 20_i64) + 1) as u8;
        emit!(DailyBugEvent { bug_id });

        Ok(())
    }
}

pub fn handler(ctx: Context<GetDailyBug>) -> Result<()> {
    ctx.accounts.get_daily_bug()
}
