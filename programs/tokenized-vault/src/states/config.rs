use anchor_lang::prelude::*;

#[account(zero_copy)]
#[derive(InitSpace)]
pub struct Config {
    pub owner: Pubkey,
    pub asset_mint: Pubkey,
    pub decimals_offset: u8,
    pub _padding1: [u8; 7],
    pub _padding2: [u64; 30],
}

#[event]
pub struct ConfigInitialized {
    pub owner: Pubkey,
}
