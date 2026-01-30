use anchor_lang::prelude::*;

declare_id!("FD9zYpxBF4s7sEWqakb3RHngswJd2DKVr7HEXq1QBndb");

#[program]
pub mod vesting_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
