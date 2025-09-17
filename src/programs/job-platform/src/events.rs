use anchor_lang::prelude::*;
use crate::state::ApplicationStatus;

#[event]
pub struct ProfileCreated {
    pub owner: Pubkey,
    pub created_at: i64,
}

#[event]
pub struct JobCreated {
    pub job_id: Pubkey,
    pub recruiter: Pubkey,
    pub title: String,
    pub created_at: i64,
}

#[event]
pub struct JobApplication {
    pub application_id: Pubkey,
    pub job_id: Pubkey,
    pub applicant: Pubkey,
    pub applied_at: i64,
}

#[event]
pub struct ApplicationStatusUpdated {
    pub application_id: Pubkey,
    pub new_status: ApplicationStatus,
}

#[event]
pub struct ScoutOfferSent {
    pub scout_offer_id: Pubkey,
    pub recruiter: Pubkey,
    pub target_profile: Pubkey,
    pub incentive_amount: u64,
}

#[event]
pub struct ScoutOfferResponded {
    pub scout_offer_id: Pubkey,
    pub accepted: bool,
}