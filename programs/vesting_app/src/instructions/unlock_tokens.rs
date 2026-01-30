use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Transfer},
    token_interface::{TokenAccount, TokenInterface},
};

use crate::state::Vesting;

#[derive(Accounts)]
pub struct UnlockTokens<'info> {
    #[account(mut,constraint=signer.key()==vesting_account.receiver)]
    pub signer: Signer<'info>,

    #[account(mut,seeds=[b"vesting",vesting_account.receiver.as_ref(),vesting_account.token_mint.as_ref()],bump=vesting_account.bump)]
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
    let vesting = &mut ctx.accounts.vesting_account;
    let now = Clock::get()?.unix_timestamp;

    if Clock::get()?.unix_timestamp < vesting.cliff_time {
        return Ok(());
    }

    let total_vested = if now >= vesting.end_time {
        vesting.amount
    } else {
        let elapsed = now - vesting.start_time;
        let duration = vesting.end_time - vesting.start_time;

        (vesting.amount as u128 * elapsed as u128 / duration as u128) as u64
    };
    let claimable = total_vested.saturating_sub(vesting.released);
    let seeds: &[&[&[u8]]] = &[&[
        b"vesting",
        vesting.receiver.as_ref(),
        vesting.token_mint.as_ref(),
        &[vesting.bump],
    ]];

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vesting_token_account.to_account_info(),
            to: ctx.accounts.receiver_token_account.to_account_info(),
            authority: vesting.to_account_info(),
        },
    )
    .with_signer(seeds);

    token::transfer(cpi_ctx, claimable)?;

    // 4. Update state
    vesting.released += claimable;
    Ok(())
}
