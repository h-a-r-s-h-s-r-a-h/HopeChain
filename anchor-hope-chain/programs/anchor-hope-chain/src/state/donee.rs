use anchor_lang::prelude::*;

#[account]
pub struct DoneeAccountState {
    pub fund_id: String,
    pub fund_generator: Pubkey,
    pub name_of_fund: String,
    pub accountNo: PubKey,
    pub name_of_fund_raiser: String,
    pub aadhar_no_of_fund_raiser: u64,
    pub description_of_problem: String,
    pub email_of_fundraiser: String,
    pub phone_of_fundraiser: String,
    pub total_amount_needed: u32,
    pub total_amount_collected: u32,
    pub created_at: i64,
    pub is_active: bool,
}
