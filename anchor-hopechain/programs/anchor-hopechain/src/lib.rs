use anchor_lang::prelude::*;

declare_id!("5dpbuSvqDGKWpL8oeWS31rbDuJcQFHGHYDqooLGaTzMN");

#[program]
pub mod anchor_hopechain {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
