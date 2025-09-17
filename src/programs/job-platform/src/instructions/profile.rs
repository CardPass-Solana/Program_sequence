use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;
use crate::events::*;

/// 프로필 생성
pub fn create_profile(
    ctx: Context<CreateProfile>,
    skills: Vec<String>,
    experience_years: u16,
    region: String,
    bio: String,
) -> Result<()> {
    let profile = &mut ctx.accounts.profile;
    let clock = Clock::get()?;

    require!(skills.len() <= 10, JobPlatformError::TooManySkills);
    require!(bio.len() <= 500, JobPlatformError::BioTooLong);

    profile.owner = ctx.accounts.owner.key();
    profile.skills = skills;
    profile.experience_years = experience_years;
    profile.region = region;
    profile.bio = bio;
    profile.created_at = clock.unix_timestamp;
    profile.is_public = true;
    profile.bump = ctx.bumps.profile;

    emit!(ProfileCreated {
        owner: profile.owner,
        created_at: profile.created_at,
    });

    Ok(())
}

/// 프로필 업데이트
pub fn update_profile(
    ctx: Context<UpdateProfile>,
    skills: Option<Vec<String>>,
    bio: Option<String>,
    is_public: Option<bool>,
) -> Result<()> {
    let profile = &mut ctx.accounts.profile;

    if let Some(skills) = skills {
        require!(skills.len() <= 10, JobPlatformError::TooManySkills);
        profile.skills = skills;
    }

    if let Some(bio) = bio {
        require!(bio.len() <= 500, JobPlatformError::BioTooLong);
        profile.bio = bio;
    }

    if let Some(is_public) = is_public {
        profile.is_public = is_public;
    }

    Ok(())
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + Profile::INIT_SPACE,
        seeds = [b"profile", owner.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, Profile>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    #[account(
        mut,
        seeds = [b"profile", owner.key().as_ref()],
        bump = profile.bump,
        has_one = owner
    )]
    pub profile: Account<'info, Profile>,

    pub owner: Signer<'info>,
}