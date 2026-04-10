use anchor_lang::prelude::*;

declare_id!("Wd48Y14eLM4zQmLWyEnZ12whBirZeUSNSTNbNW7HPBa");

#[program]
pub mod lords_mock_usdc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
