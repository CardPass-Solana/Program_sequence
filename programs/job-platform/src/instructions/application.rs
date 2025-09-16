use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;
use crate::events::*;

/// Job에 지원하기
pub fn apply_to_job(
    ctx: Context<ApplyToJob>,
    cover_letter: String,
) -> Result<()> {
    let job = &mut ctx.accounts.job;
    let application = &mut ctx.accounts.application;
    let profile = &ctx.accounts.profile;
    let clock = Clock::get()?;

    require!(job.is_active, JobPlatformError::JobNotActive);
    require!(clock.unix_timestamp < job.deadline, JobPlatformError::JobExpired);
    require!(cover_letter.len() <= 1000, JobPlatformError::CoverLetterTooLong);
    require!(profile.is_public, JobPlatformError::ProfileNotPublic);

    application.applicant = ctx.accounts.applicant.key();
    application.job = job.key();
    application.profile = profile.key();
    application.cover_letter = cover_letter;
    application.applied_at = clock.unix_timestamp;
    application.status = ApplicationStatus::Pending;
    application.bump = ctx.bumps.application;

    job.application_count += 1;

    emit!(JobApplication {
        application_id: application.key(),
        job_id: job.key(),
        applicant: application.applicant,
        applied_at: application.applied_at,
    });

    Ok(())
}

/// 지원 상태 업데이트
pub fn update_application_status(
    ctx: Context<UpdateApplicationStatus>,
    new_status: ApplicationStatus,
) -> Result<()> {
    let application = &mut ctx.accounts.application;

    application.status = new_status.clone();

    emit!(ApplicationStatusUpdated {
        application_id: application.key(),
        new_status,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct ApplyToJob<'info> {
    #[account(
        init,
        payer = applicant,
        space = 8 + Application::INIT_SPACE,
        seeds = [b"application", job.key().as_ref(), applicant.key().as_ref()],
        bump
    )]
    pub application: Account<'info, Application>,

    #[account(mut)]
    pub job: Account<'info, Job>,

    #[account(
        seeds = [b"profile", applicant.key().as_ref()],
        bump = profile.bump
    )]
    pub profile: Account<'info, Profile>,

    #[account(mut)]
    pub applicant: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateApplicationStatus<'info> {
    #[account(
        mut,
        seeds = [b"application", job.key().as_ref(), application.applicant.as_ref()],
        bump = application.bump
    )]
    pub application: Account<'info, Application>,

    #[account(
        seeds = [b"job", recruiter.key().as_ref(), &job.created_at.to_le_bytes()],
        bump = job.bump,
        has_one = recruiter
    )]
    pub job: Account<'info, Job>,

    pub recruiter: Signer<'info>,
}