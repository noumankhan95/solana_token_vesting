use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Transfer},
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
    let vesting_account = &ctx.accounts.vesting_account;
    if Clock::get()?.unix_timestamp < vesting_account.start_time {
        return Ok(());
    }

    if Clock::get()?.unix_timestamp > vesting_account.end_time {
        let transfer_accounts = Transfer {
            from: ctx.accounts.vesting_token_account.to_account_info(),
            authority: ctx.accounts.vesting_account.to_account_info(),
            to: ctx.accounts.receiver_token_account.to_account_info(),
        };
        let vesting_acc_key = vesting_account.receiver.key();
        let vesting_acc_token = vesting_account.token_mint.key();
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"vesting",
            vesting_acc_key.as_ref(),
            vesting_acc_token.as_ref(),
            &[vesting_account.bump],
        ]];
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_accounts,
        )
        .with_signer(signer_seeds);
        let amount = vesting_account
            .amount
            .saturating_sub(vesting_account.released);
        token::transfer(cpi_ctx, amount)?;
    } else {
        let now = Clock::get()?.unix_timestamp;
        let elapsed = now - vesting_account.start_time;
        let duration = vesting_account
            .end_time
            .saturating_sub(vesting_account.start_time);
        let vested = (vesting_account.amount as u128 * elapsed as u128 / duration as u128) as u64;
        let transfer_accounts = Transfer {
            from: ctx.accounts.vesting_token_account.to_account_info(),
            authority: ctx.accounts.vesting_account.to_account_info(),
            to: ctx.accounts.receiver_token_account.to_account_info(),
        };
        let vesting_acc_key = vesting_account.receiver.key();
        let vesting_acc_token = vesting_account.token_mint.key();
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"vesting",
            vesting_acc_key.as_ref(),
            vesting_acc_token.as_ref(),
            &[vesting_account.bump],
        ]];
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_accounts,
        )
        .with_signer(signer_seeds);
        let amount = vesting_account
            .amount
            .saturating_sub(vesting_account.released);
        token::transfer(cpi_ctx, amount)?;
    }
    Ok(())
}
