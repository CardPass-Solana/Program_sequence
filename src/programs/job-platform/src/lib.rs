use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;
pub mod events;

use instructions::*;
use state::*;
use errors::*;
use events::*;

declare_id!("2bZY4L2gTev6uBoxBoxUefRMJFM61xay53PvPgkxeeuY");

#[program]
pub mod job_platform {
    use super::*;

    /// 프로필 생성
    pub fn create_profile(
        ctx: Context<CreateProfile>,
        skills: Vec<String>,
        experience_years: u16,
        region: String,
        bio: String,
    ) -> Result<()> {
        instructions::profile::create_profile(ctx, skills, experience_years, region, bio)
    }

    /// 프로필 업데이트
    pub fn update_profile(
        ctx: Context<UpdateProfile>,
        skills: Option<Vec<String>>,
        bio: Option<String>,
        is_public: Option<bool>,
    ) -> Result<()> {
        instructions::profile::update_profile(ctx, skills, bio, is_public)
    }

    /// Job 생성
    pub fn create_job(
        ctx: Context<CreateJob>,
        title: String,
        description: String,
        required_skills: Vec<String>,
        salary_min: u64,
        salary_max: u64,
        deadline_days: u16,
        job_id: u64,
    ) -> Result<()> {
        instructions::job::create_job(ctx, title, description, required_skills, salary_min, salary_max, deadline_days, job_id)
    }

    /// Job에 지원
    pub fn apply_to_job(
        ctx: Context<ApplyToJob>,
        cover_letter: String,
    ) -> Result<()> {
        instructions::application::apply_to_job(ctx, cover_letter)
    }

    /// 지원 상태 업데이트
    pub fn update_application_status(
        ctx: Context<UpdateApplicationStatus>,
        new_status: ApplicationStatus,
    ) -> Result<()> {
        instructions::application::update_application_status(ctx, new_status)
    }

    /// 스카우트 제안 보내기
    pub fn send_scout_offer(
        ctx: Context<SendScoutOffer>,
        message: String,
        incentive_amount: u64,
    ) -> Result<()> {
        instructions::scout::send_scout_offer(ctx, message, incentive_amount)
    }

    /// 스카우트 제안에 응답
    pub fn respond_to_scout(
        ctx: Context<RespondToScout>,
        accept: bool,
    ) -> Result<()> {
        instructions::scout::respond_to_scout(ctx, accept)
    }
}