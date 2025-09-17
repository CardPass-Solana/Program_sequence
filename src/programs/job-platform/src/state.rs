use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Profile {
    pub owner: Pubkey,
    #[max_len(10, 50)]
    pub skills: Vec<String>,
    pub experience_years: u16,
    #[max_len(50)]
    pub region: String,
    #[max_len(500)]
    pub bio: String,
    pub created_at: i64,
    pub is_public: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Job {
    pub recruiter: Pubkey,
    #[max_len(100)]
    pub title: String,
    #[max_len(1000)]
    pub description: String,
    #[max_len(10, 50)]
    pub required_skills: Vec<String>,
    pub salary_min: u64,
    pub salary_max: u64,
    pub created_at: i64,
    pub deadline: i64,
    pub is_active: bool,
    pub application_count: u32,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Application {
    pub applicant: Pubkey,
    pub job: Pubkey,
    pub profile: Pubkey,
    #[max_len(1000)]
    pub cover_letter: String,
    pub applied_at: i64,
    pub status: ApplicationStatus,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct ScoutOffer {
    pub recruiter: Pubkey,
    pub target_profile: Pubkey,
    #[max_len(500)]
    pub message: String,
    pub incentive_amount: u64,
    pub created_at: i64,
    pub expires_at: i64,
    pub status: ScoutStatus,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub enum ApplicationStatus {
    Pending,
    Reviewing,
    Interview,
    Accepted,
    Rejected,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub enum ScoutStatus {
    Pending,
    Accepted,
    Rejected,
    Expired,
}