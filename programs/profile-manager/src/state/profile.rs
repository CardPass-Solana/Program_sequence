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
    #[max_len(30)]
    pub handle: String,
    pub created_at: i64,
    pub is_public: bool,
    #[max_len(5)]
    pub contact_prices: Vec<ContactPriceTier>,
    pub response_time_hours: u16,
    pub nft_mint: Option<Pubkey>,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct ContactRequest {
    pub requester: Pubkey,
    pub target_profile: Pubkey,
    #[max_len(1000)]
    pub message: String,
    pub amount: u64,
    pub created_at: i64,
    pub expires_at: i64,
    pub status: ContactStatus,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub enum ContactStatus {
    Pending,
    Responded,
    Rejected,
    Expired,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct ContactPriceTier {
    pub price: u64,
    #[max_len(50)]
    pub description: String,
}