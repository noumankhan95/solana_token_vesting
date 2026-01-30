use anchor_lang::prelude::*;
mod error;
mod instructions;
mod state;
declare_id!("FD9zYpxBF4s7sEWqakb3RHngswJd2DKVr7HEXq1QBndb");

#[program]
pub mod vesting_app {
    pub use super::instructions::*;
    use super::*;
    pub fn initialize(
        ctx: Context<LockTokens>,
        amount: u64,
        start_time: i64,
        end_time: i64,
        cliff_time: i64,
        receiver: Pubkey,
    ) -> Result<()> {
        initialize_vesting(ctx, amount, start_time, end_time, cliff_time, receiver)?;
        Ok(())
    }

    pub fn init_unlock_tokens(ctx: Context<UnlockTokens>) -> Result<()> {
        unlock_tokens(ctx)?;
        Ok(())
    }
}
