use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;
use crate::events::*;

/// 스카우트 제안 보내기
pub fn send_scout_offer(
    ctx: Context<SendScoutOffer>,
    message: String,
    incentive_amount: u64,
) -> Result<()> {
    let clock = Clock::get()?;

    require!(message.len() <= 500, JobPlatformError::MessageTooLong);
    require!(incentive_amount >= 1_000_000, JobPlatformError::IncentiveTooLow); // 0.001 SOL minimum

    // Transfer SOL from recruiter to scout offer account
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        anchor_lang::system_program::Transfer {
            from: ctx.accounts.recruiter.to_account_info(),
            to: ctx.accounts.scout_offer.to_account_info(),
        },
    );
    anchor_lang::system_program::transfer(cpi_context, incentive_amount)?;

    let scout_offer = &mut ctx.accounts.scout_offer;
    scout_offer.recruiter = ctx.accounts.recruiter.key();
    scout_offer.target_profile = ctx.accounts.target_profile.key();
    scout_offer.message = message;
    scout_offer.incentive_amount = incentive_amount;
    scout_offer.created_at = clock.unix_timestamp;
    scout_offer.expires_at = clock.unix_timestamp + (7 * 24 * 60 * 60); // 7 days
    scout_offer.status = ScoutStatus::Pending;
    scout_offer.bump = ctx.bumps.scout_offer;

    emit!(ScoutOfferSent {
        scout_offer_id: scout_offer.key(),
        recruiter: scout_offer.recruiter,
        target_profile: scout_offer.target_profile,
        incentive_amount,
    });

    Ok(())
}

/// 스카우트 제안에 응답
pub fn respond_to_scout(
    ctx: Context<RespondToScout>,
    accept: bool,
) -> Result<()> {
    let clock = Clock::get()?;

    require!(ctx.accounts.scout_offer.status == ScoutStatus::Pending, JobPlatformError::ScoutOfferNotPending);
    require!(clock.unix_timestamp < ctx.accounts.scout_offer.expires_at, JobPlatformError::ScoutOfferExpired);

    let incentive_amount = ctx.accounts.scout_offer.incentive_amount;
    let scout_offer_key = ctx.accounts.scout_offer.key();

    if accept {
        ctx.accounts.scout_offer.status = ScoutStatus::Accepted;

        // Transfer incentive to profile owner using System Program
        let transfer_instruction = anchor_lang::system_program::Transfer {
            from: ctx.accounts.scout_offer.to_account_info(),
            to: ctx.accounts.profile_owner.to_account_info(),
        };
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            transfer_instruction,
        );
        anchor_lang::system_program::transfer(cpi_context, incentive_amount)?;
    } else {
        ctx.accounts.scout_offer.status = ScoutStatus::Rejected;

        // Refund to recruiter using System Program
        let transfer_instruction = anchor_lang::system_program::Transfer {
            from: ctx.accounts.scout_offer.to_account_info(),
            to: ctx.accounts.recruiter.to_account_info(),
        };
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            transfer_instruction,
        );
        anchor_lang::system_program::transfer(cpi_context, incentive_amount)?;
    }

    emit!(ScoutOfferResponded {
        scout_offer_id: scout_offer_key,
        accepted: accept,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct SendScoutOffer<'info> {
    #[account(
        init,
        payer = recruiter,
        space = 8 + ScoutOffer::INIT_SPACE,
        seeds = [b"scout", recruiter.key().as_ref(), target_profile.key().as_ref()],
        bump
    )]
    pub scout_offer: Account<'info, ScoutOffer>,

    #[account(
        seeds = [b"profile", target_profile.owner.as_ref()],
        bump = target_profile.bump
    )]
    pub target_profile: Account<'info, Profile>,

    #[account(mut)]
    pub recruiter: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RespondToScout<'info> {
    #[account(
        mut,
        seeds = [b"scout", scout_offer.recruiter.as_ref(), profile.key().as_ref()],
        bump = scout_offer.bump
    )]
    pub scout_offer: Account<'info, ScoutOffer>,

    #[account(
        seeds = [b"profile", profile_owner.key().as_ref()],
        bump = profile.bump
    )]
    pub profile: Account<'info, Profile>,

    #[account(mut)]
    pub profile_owner: Signer<'info>,

    /// CHECK: recruiter account for refunds
    #[account(mut)]
    pub recruiter: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}