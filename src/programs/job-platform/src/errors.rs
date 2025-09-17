use anchor_lang::prelude::*;

#[error_code]
pub enum JobPlatformError {
    #[msg("Too many skills provided (max 10)")]
    TooManySkills,
    #[msg("Bio is too long (max 500 characters)")]
    BioTooLong,
    #[msg("Title is too long (max 100 characters)")]
    TitleTooLong,
    #[msg("Description is too long (max 1000 characters)")]
    DescriptionTooLong,
    #[msg("Invalid salary range (max must be >= min)")]
    InvalidSalaryRange,
    #[msg("Invalid deadline (must be 1-365 days)")]
    InvalidDeadline,
    #[msg("Job is not active")]
    JobNotActive,
    #[msg("Job has expired")]
    JobExpired,
    #[msg("Cover letter is too long (max 1000 characters)")]
    CoverLetterTooLong,
    #[msg("Profile must be public to apply for jobs")]
    ProfileNotPublic,
    #[msg("Message is too long (max 500 characters)")]
    MessageTooLong,
    #[msg("Incentive amount too low (minimum 0.001 SOL)")]
    IncentiveTooLow,
    #[msg("Scout offer is not pending")]
    ScoutOfferNotPending,
    #[msg("Scout offer has expired")]
    ScoutOfferExpired,
}