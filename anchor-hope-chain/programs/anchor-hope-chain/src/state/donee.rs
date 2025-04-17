use anchor_lang::prelude::*;

#[account]
pub struct DoneeAccountState {
    pub fund_generator: Pubkey,
    pub accountNo: PubKey,
    pub name_of_fund_raiser: String,
    pub aadhar_no_of_fund_raiser: u64,
    pub description_of_problem: String,
    pub target: u32,
    pub created_at: i64,
    pub is_active: bool,
}
