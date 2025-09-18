use anchor_lang::prelude::*;

#[event]
pub struct ProfileCreated {
    pub owner: Pubkey,
    pub handle: String,
    pub created_at: i64,
}

#[event]
pub struct ContactRequestSent {
    pub requester: Pubkey,
    pub target: Pubkey,
    pub amount: u64,
    pub created_at: i64,
}

#[event]
pub struct ContactRequestProcessed {
    pub requester: Pubkey,
    pub target: Pubkey,
    pub accepted: bool,
    pub amount: u64,
}

#[event]
pub struct ContactRequestExpired {
    pub requester: Pubkey,
    pub target: Pubkey,
    pub amount: u64,
}