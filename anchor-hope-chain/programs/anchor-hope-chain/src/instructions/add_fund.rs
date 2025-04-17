use crate::{constants::*, contexts::*, errors::*};
use anchor_lang::prelude::*;

pub fn create_fund(
    ctx: Context<CreateFund>,
    fund_id: String,
    name_of_fund: String,
    account_no: Pubkey,
    name_of_fund_raiser: String,
    aadhar_no_of_fund_raiser: u64,
    description_of_problem: String,
    email_of_fundraiser: String,
    phone_of_fundraiser: String,
    total_amount_needed: u32,
) -> Result<()> {
    require!(
        fund_id.len() <= MAX_ID_LENGTH,
        DoneeAccountError::FundIdTooLong
    );

    require!(
        name_of_fund.len() <= MAX_FUND_NAME,
        DoneeAccountError::FundNameTooLong
    );

    require!(
        name_of_fund_raiser.len() <= MAX_GENERATOR_NAME,
        DoneeAccountError::NameOfFundRaiserTooLong
    );

    require!(
        description_of_problem.len() <= MAX_TOPIC_DESCRIPTION,
        DoneeAccountError::DescriptionTooLong
    );

    require!(
        email_of_fundraiser.len() <= MAX_EMAIL_SIZE,
        DoneeAccountError::EmailOfFundRaiserTooLong
    );

    require!(
        phone_of_fundraiser.len() <= MAX_PHONE_SIZE,
        DoneeAccountError::PhoneOfFundRaiserTooLong
    );

    let donee = &mut ctx.accounts.donee;
    donee.fund_id = fund_id;
    donee.fund_generator = ctx.accounts.fund_generator.key();
    donee.name_of_fund = name_of_fund;
    donee.account_no = account_no;
    donee.name_of_fund_raiser = name_of_fund_raiser;
    donee.aadhar_no_of_fund_raiser = aadhar_no_of_fund_raiser;
    donee.description_of_problem = description_of_problem;
    donee.email_of_fundraiser = email_of_fundraiser;
    donee.phone_of_fundraiser = phone_of_fundraiser;
    donee.total_amount_needed = total_amount_needed;
    donee.total_amount_collected = 0;
    donee.created_at = Clock::get()?.unix_timestamp;
    donee.is_active = true;
    Ok(())
}
