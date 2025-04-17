use anchor_lang::prelude::*;

declare_id!("5iPJovG7C9NA8rqdBmqUb26XFTtNBuzmG2bTnYFV1C5a");

#[program]
pub mod anchor_hope_chain {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
