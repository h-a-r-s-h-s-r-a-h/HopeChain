use crate::{constants::*, state::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(fund_id:String)]
pub struct CreateDonator<'info> {
    #[account(
        mut,
        seeds=["donee".as_bytes(), fund_id.as_bytes()],
        bump
    )]
    pub donee: Account<'info, DoneeAccountState>,

    #[account(
        init,
        seeds=["donate".as_bytes(), fund_id.as_bytes(), donator.key.as_ref()],
        bump,
        payer=donator,
        space=DonatorAccountState::INIT_SPACE + MAX_ID_LENGTH + MAX_BLESSING_MESSAGE_SIZE + MAX_GENERATOR_NAME 
        + MAX_EMAIL_SIZE + MAX_PHONE_SIZE
    )]
    pub donate: Account<'info, DonatorAccountState>,

    #[account(mut)]
    pub donator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl Space for DonatorAccountState {
    const INIT_SPACE: usize = ANCHOR_DISCRIMINATOR
        + PUBKEY_SIZE
        + STRING_LENGTH_PREFIX
        + STRING_LENGTH_PREFIX
        + STRING_LENGTH_PREFIX
        + U32_SIZE
        + STRING_LENGTH_PREFIX
        + U32_SIZE
        + STRING_LENGTH_PREFIX
        + I64_SIZE;
}
