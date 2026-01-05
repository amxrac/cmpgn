use crate::error::ErrorCode;
use crate::DailyBug;
use anchor_lang::prelude::*;
use anchor_lang::{
    solana_program::{account_info::AccountInfo, program_error::ProgramError},
    AccountDeserialize,
};

use orao_solana_vrf::cpi::accounts::RequestV2;
use orao_solana_vrf::program::OraoVrf;
use orao_solana_vrf::state::{NetworkState, RandomnessAccountData};
use orao_solana_vrf::CONFIG_ACCOUNT_SEED;
use orao_solana_vrf::RANDOMNESS_ACCOUNT_SEED;

#[event]
pub struct DailyBugEvent {
    pub bug_id: u8,
}

#[derive(Accounts)]
pub struct ConsumeDailyBug<'info> {
    #[account(
        mut,
        seeds = [b"daily_bug_seed"],
        bump,
    )]
    pub bug_state: Account<'info, DailyBug>,

    /// CHECK:
    #[account(
        seeds = [RANDOMNESS_ACCOUNT_SEED, bug_state.force.as_ref()],
        bump,
        seeds::program = orao_solana_vrf::ID
    )]
    pub random: UncheckedAccount<'info>,
}

impl<'info> ConsumeDailyBug<'info> {
    pub fn consume_daily_bug(&mut self) -> Result<()> {
        require!(self.bug_state.force != [0; 32], ErrorCode::BugNotRequested);
        require!(
            self.bug_state.bug_id.is_none(),
            ErrorCode::BugAlreadyConsumed
        );

        let rand = get_account_data(&self.random)?;

        let randomness = rand
            .fulfilled_randomness()
            .ok_or(ErrorCode::RandomnessNotFulfilled)?;

        let bug_id = (randomness[0] % 20) + 1;

        self.bug_state.bug_id = Some(bug_id);

        emit!(DailyBugEvent { bug_id });
        Ok(())
    }
}

pub fn get_account_data(account_info: &AccountInfo) -> Result<RandomnessAccountData> {
    if account_info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount.into());
    }

    RandomnessAccountData::try_deserialize(&mut &account_info.data.borrow()[..])
        .map_err(|e| e.into())
}

pub fn handler(ctx: Context<ConsumeDailyBug>) -> Result<()> {
    ctx.accounts.consume_daily_bug()
}
