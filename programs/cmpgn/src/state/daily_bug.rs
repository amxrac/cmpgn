use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct DailyBug {
    pub bug_id: Option<u8>,
    pub day: i64,
    pub force: [u8; 32],
}
