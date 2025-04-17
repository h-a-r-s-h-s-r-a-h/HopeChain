use anchor_lang::prelude::*;

#[error_code]
pub enum DoneeAccountError {
    #[msg("Id Too Long")]
    FundIdTooLong,
    #[msg("Name of Fund Too Long")]
    FundNameTooLong,
    #[msg("Name of Fund Raiser Too Long")]
    NameOfFundRaiserTooLong,
    #[msg("Email of Fund Raiser Too Long")]
    EmailOfFundRaiserTooLong,
    #[msg("Phone of Fund Raiser Too Long")]
    PhoneOfFundRaiserTooLong,
    #[msg("Description Too long")]
    DescriptionTooLong,
    #[msg("Poll is not active")]
    PollNotActive,
    #[msg("Meet the Requirement earlier!")]
    RequirementMeetEarlier,
}
