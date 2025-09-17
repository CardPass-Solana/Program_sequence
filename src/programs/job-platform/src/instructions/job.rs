use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;
use crate::events::*;

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
    let job = &mut ctx.accounts.job;
    let clock = Clock::get()?;

    require!(title.len() <= 100, JobPlatformError::TitleTooLong);
    require!(description.len() <= 1000, JobPlatformError::DescriptionTooLong);
    require!(required_skills.len() <= 10, JobPlatformError::TooManySkills);
    require!(salary_max >= salary_min, JobPlatformError::InvalidSalaryRange);
    require!(deadline_days > 0 && deadline_days <= 365, JobPlatformError::InvalidDeadline);

    job.recruiter = ctx.accounts.recruiter.key();
    job.title = title;
    job.description = description;
    job.required_skills = required_skills;
    job.salary_min = salary_min;
    job.salary_max = salary_max;
    job.created_at = clock.unix_timestamp;
    job.deadline = clock.unix_timestamp + (deadline_days as i64 * 24 * 60 * 60);
    job.is_active = true;
    job.application_count = 0;
    job.bump = ctx.bumps.job;

    emit!(JobCreated {
        job_id: job.key(),
        recruiter: job.recruiter,
        title: job.title.clone(),
        created_at: job.created_at,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(title: String, description: String, required_skills: Vec<String>, salary_min: u64, salary_max: u64, deadline_days: u16, job_id: u64)]
pub struct CreateJob<'info> {
    #[account(
        init,
        payer = recruiter,
        space = 8 + Job::INIT_SPACE,
        seeds = [b"job", recruiter.key().as_ref(), &job_id.to_le_bytes()],
        bump
    )]
    pub job: Account<'info, Job>,

    #[account(mut)]
    pub recruiter: Signer<'info>,

    pub system_program: Program<'info, System>,
}