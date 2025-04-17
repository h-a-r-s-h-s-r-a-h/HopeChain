use anchor_lang::prelude::*;

#[account]
pub struct DonatorAccountState {
    pub donator_pubkey: Pubkey,
    pub fund_id: String,
    pub blessing_message: String,
    pub donator_name: String,
    pub total_donated_amount: u32,
    pub email_of_donator: String,
    pub phone_of_donator: String,
    pub funded_at: i64,
}
