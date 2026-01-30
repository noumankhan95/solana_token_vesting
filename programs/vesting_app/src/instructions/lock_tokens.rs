use anchor_lang::prelude::*;

use crate::state::Vesting;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Transfer},
    token_interface::{Mint, TokenAccount, TokenInterface},
};
#[derive(Accounts)]
#[instruction(receiver:Pubkey)]
pub struct LockTokens<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(init,payer=signer,space=8+Vesting::INIT_SPACE,seeds=[b"vesting",receiver.key().as_ref(),token_to_vest_mint.key().as_ref()],bump)]
    pub vesting_account: Account<'info, Vesting>,
    #[account(mut,associated_token::mint=vesting_account.token_mint,associated_token::authority=vesting_account)]
    pub vesting_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_to_vest_mint: InterfaceAccount<'info, Mint>,
    pub vested_tokens: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_vesting(
    ctx: Context<LockTokens>,
    amount: u64,
    start_time: i64,
    end_time: i64,
    cliff_time: i64,
    receiver: Pubkey,
) -> Result<()> {
    let transfer_acc = Transfer {
        authority: ctx.accounts.signer.to_account_info(),
        from: ctx.accounts.vested_tokens.to_account_info(),
        to: ctx.accounts.vesting_token_account.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer_acc);
    token::transfer(cpi_ctx, amount)?;
    let vesting_acc = &mut ctx.accounts.vesting_account;
    vesting_acc.amount = amount;
    vesting_acc.cliff_time = cliff_time;
    vesting_acc.start_time = start_time;
    vesting_acc.end_time = end_time;
    vesting_acc.receiver = receiver;
    vesting_acc.token_mint = ctx.accounts.token_to_vest_mint.key();
    vesting_acc.bump = ctx.bumps.vesting_account;
    vesting_acc.released = 0;
    Ok(())
}
