use anchor_lang::prelude::*;

#[account(zero_copy)]
#[derive(InitSpace)]
pub struct Config {
    pub owner: Pubkey,
    pub fee_recipient: Pubkey,
    pub creation_fee: u64, // sol
    pub _padding1: [u8; 8],
    pub _padding2: [u64; 30],
}

#[event]
pub struct ConfigInitialized {
    pub owner: Pubkey,
    pub fee_recipient: Pubkey,
    pub creation_fee: u64,
}

#[event]
pub struct FeeRecipientUpdated {
    pub fee_recipient: Pubkey,
}

#[event]
pub struct CreationFeeUpdated {
    pub creation_fee: u64,
}
