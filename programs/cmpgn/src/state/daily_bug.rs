use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct DailyBug {
    pub bug_id: Option<u8>,
    pub day: i8,
    pub seed: [u8; 32],
    pub requested_at: i64,
}
