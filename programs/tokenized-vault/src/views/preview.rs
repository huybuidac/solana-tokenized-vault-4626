use anchor_lang::prelude::*;

use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount, TokenInterface};

use crate::states::Config;
use crate::utils::{
    preview_deposit_internal, preview_mint_internal, preview_redeem_internal,
    preview_withdraw_internal, CONFIG_SEED, SHARES_MINT_SEED,
};

#[derive(Accounts)]
pub struct Preview<'info> {
    #[account(mint::token_program = asset_token_program)]
    pub asset_mint: InterfaceAccount<'info, Mint>,

    #[account(associated_token::mint = asset_mint, associated_token::authority = shares_mint, associated_token::token_program = asset_token_program)]
    pub asset_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(seeds = [SHARES_MINT_SEED.as_bytes()], bump, mint::token_program = token2022_program)]
    pub shares_mint: InterfaceAccount<'info, Mint>,

    #[account(seeds = [CONFIG_SEED.as_bytes()], bump, has_one = asset_mint)]
    pub config: AccountLoader<'info, Config>,

    pub asset_token_program: Interface<'info, TokenInterface>,
    pub token2022_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn process_preview_deposit(ctx: Context<Preview>, assets: u64) -> Result<u64> {
    let shares = preview_deposit_internal(
        assets,
        ctx.accounts.asset_vault.amount,
        ctx.accounts.shares_mint.supply,
        ctx.accounts.config.load()?.decimals_offset,
    )?;
    Ok(shares)
}

pub fn process_preview_mint(ctx: Context<Preview>, shares: u64) -> Result<u64> {
    let assets = preview_mint_internal(
        shares,
        ctx.accounts.asset_vault.amount,
        ctx.accounts.shares_mint.supply,
        ctx.accounts.config.load()?.decimals_offset,
    )?;
    Ok(assets)
}

pub fn process_preview_withdraw(ctx: Context<Preview>, assets: u64) -> Result<u64> {
    let shares = preview_withdraw_internal(
        assets,
        ctx.accounts.asset_vault.amount,
        ctx.accounts.shares_mint.supply,
        ctx.accounts.config.load()?.decimals_offset,
    )?;
    Ok(shares)
}

pub fn process_preview_redeem(ctx: Context<Preview>, shares: u64) -> Result<u64> {
    let assets = preview_redeem_internal(
        shares,
        ctx.accounts.asset_vault.amount,
        ctx.accounts.shares_mint.supply,
        ctx.accounts.config.load()?.decimals_offset,
    )?;
    Ok(assets)
}
