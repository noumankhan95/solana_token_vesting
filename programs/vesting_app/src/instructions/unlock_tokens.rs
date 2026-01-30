use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{TokenAccount, TokenInterface},
};

use crate::state::Vesting;

#[derive(Accounts)]
pub struct UnlockTokens<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut,seeds=[b"vesting",vesting_account.receiver.key().as_ref(),vesting_account.token_mint.key().as_ref()],bump=vesting_account.bump)]
    pub vesting_account: Account<'info, Vesting>,
    #[account(mut,associated_token::mint=vesting_account.token_mint,associated_token::authority=vesting_account)]
    pub vesting_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut,associated_token::mint=vesting_account.token_mint,associated_token::authority=vesting_account.receiver)]
    pub receiver_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,
}

pub fn unlock_tokens(ctx: Context<UnlockTokens>) -> Result<()> {
    Ok(())
}
