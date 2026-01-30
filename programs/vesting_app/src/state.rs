use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Vesting {
    pub receiver: Pubkey,
    pub token_mint: Pubkey,
    pub start_time: i64,
    pub cliff_time: i64,
    pub end_time: i64,
    pub released: u64,
    pub amount: u64,
    pub bump: u8,
}
