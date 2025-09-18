use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;
use crate::events::*;

pub fn create_profile(
    ctx: Context<CreateProfile>,
    skills: Vec<String>,
    experience_years: u16,
    region: String,
    bio: String,
    handle: String,
    contact_prices: Vec<ContactPriceTier>,
    response_time_hours: u16,
) -> Result<()> {
    let profile = &mut ctx.accounts.profile;
    let clock = Clock::get()?;

    require!(skills.len() <= 10, ProfileManagerError::TooManySkills);
    require!(bio.len() <= 500, ProfileManagerError::BioTooLong);
    require!(handle.len() >= 3 && handle.len() <= 30, ProfileManagerError::InvalidHandle);
    require!(response_time_hours > 0 && response_time_hours <= 168, ProfileManagerError::InvalidResponseTime);

    profile.owner = ctx.accounts.owner.key();
    profile.skills = skills;
    profile.experience_years = experience_years;
    profile.region = region;
    profile.bio = bio;
    profile.handle = handle.to_lowercase();
    profile.contact_prices = contact_prices;
    profile.response_time_hours = response_time_hours;
    profile.created_at = clock.unix_timestamp;
    profile.is_public = true;
    profile.nft_mint = None;
    profile.bump = ctx.bumps.profile;

    emit!(ProfileCreated {
        owner: profile.owner,
        handle: profile.handle.clone(),
        created_at: profile.created_at,
    });

    Ok(())
}

pub fn update_profile(
    ctx: Context<UpdateProfile>,
    skills: Option<Vec<String>>,
    bio: Option<String>,
    is_public: Option<bool>,
    contact_prices: Option<Vec<ContactPriceTier>>,
    response_time_hours: Option<u16>,
) -> Result<()> {
    let profile = &mut ctx.accounts.profile;

    if let Some(skills) = skills {
        require!(skills.len() <= 10, ProfileManagerError::TooManySkills);
        profile.skills = skills;
    }

    if let Some(bio) = bio {
        require!(bio.len() <= 500, ProfileManagerError::BioTooLong);
        profile.bio = bio;
    }

    if let Some(is_public) = is_public {
        profile.is_public = is_public;
    }

    if let Some(contact_prices) = contact_prices {
        profile.contact_prices = contact_prices;
    }

    if let Some(response_time_hours) = response_time_hours {
        require!(response_time_hours > 0 && response_time_hours <= 168, ProfileManagerError::InvalidResponseTime);
        profile.response_time_hours = response_time_hours;
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(skills: Vec<String>, experience_years: u16, region: String, bio: String, handle: String)]
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