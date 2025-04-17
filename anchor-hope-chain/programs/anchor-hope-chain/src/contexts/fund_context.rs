use crate::{constants::*, state::DoneeAccountState};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(fund_id:String)]
pub struct CreateFund<'info> {
    #[account(
        init,
        seeds=["donee".as_bytes(), fund_id.as_bytes(), fund_generator.key.as_ref()],
        bump,
        payer=fund_generator,
        space=DoneeAccountState::INIT_SPACE + MAX_ID_LENGTH + MAX_FUND_NAME + MAX_GENERATOR_NAME + MAX_TOPIC_DESCRIPTION + MAX_EMAIL_SIZE + MAX_PHONE_SIZE
    )]
    pub donee: Account<'info, DoneeAccountState>,

    #[account(mut)]
    pub fund_generator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// pub fund_id: String,//
// pub fund_generator: Pubkey,//
// pub name_of_fund: String,//
// pub accountNo: PubKey,//
// pub name_of_fund_raiser: String,//
// pub aadhar_no_of_fund_raiser: u64,//
// pub description_of_problem: String,//
// pub email_of_fundraiser: String,//
// pub phone_of_fundraiser: String,//
// pub total_amount_needed: u32,
// pub total_amount_collected: u32,
// pub created_at: i64,
// pub is_active: bool,/

impl Space for DoneeAccountState {
    const INIT_SPACE: usize = ANCHOR_DISCRIMINATOR
        + STRING_LENGTH_PREFIX
        + PUBKEY_SIZE
        + STRING_LENGTH_PREFIX
        + PUBKEY_SIZE
        + STRING_LENGTH_PREFIX
        + U64_SIZE
        + STRING_LENGTH_PREFIX
        + STRING_LENGTH_PREFIX
        + STRING_LENGTH_PREFIX
        + U32_SIZE
        + U32_SIZE
        + I64_SIZE
        + BOOL_SIZE;
}
