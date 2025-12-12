use anchor_lang::prelude::*;

use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    mint_to_checked, transfer_checked, Mint, MintToChecked, Token2022, TokenAccount,
    TokenInterface, TransferChecked,
};

use crate::states::Config;
use crate::utils::{
    preview_deposit_internal, preview_mint_internal, CONFIG_SEED, SHARES_MINT_SEED,
};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut, mint::token_program = asset_token_program)]
    pub asset_mint: InterfaceAccount<'info, Mint>,
    #[account(mut, associated_token::mint = asset_mint, associated_token::authority = user, associated_token::token_program = asset_token_program)]
    pub user_asset_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(mut, associated_token::mint = asset_mint, associated_token::authority = shares_mint, associated_token::token_program = asset_token_program)]
    pub asset_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(mut, seeds = [SHARES_MINT_SEED.as_bytes()], bump, mint::token_program = token2022_program)]
    pub shares_mint: InterfaceAccount<'info, Mint>,
    #[account(init_if_needed, payer = user, associated_token::mint = shares_mint, associated_token::authority = user, associated_token::token_program = token2022_program)]
    pub user_shares_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(seeds = [CONFIG_SEED.as_bytes()], bump, has_one = asset_mint)]
    pub config: AccountLoader<'info, Config>,

    pub asset_token_program: Interface<'info, TokenInterface>,
    pub token2022_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn process_deposit(ctx: Context<Deposit>, assets: u64) -> Result<()> {
    let shares = preview_deposit_internal(
        assets,
        ctx.accounts.asset_vault.amount,
        ctx.accounts.shares_mint.supply,
        ctx.accounts.config.load()?.decimals_offset,
    )?;
    ctx.accounts.transfer_assets_from_user_to_vault(assets)?;
    ctx.accounts
        .mint_shares_to_user(ctx.bumps.shares_mint, shares)?;
    Ok(())
}

pub fn process_mint(ctx: Context<Deposit>, shares: u64) -> Result<()> {
    let assets = preview_mint_internal(
        shares,
        ctx.accounts.asset_vault.amount,
        ctx.accounts.shares_mint.supply,
        ctx.accounts.config.load()?.decimals_offset,
    )?;
    ctx.accounts.transfer_assets_from_user_to_vault(assets)?;
    ctx.accounts
        .mint_shares_to_user(ctx.bumps.shares_mint, shares)?;
    Ok(())
}

impl<'info> Deposit<'info> {
    pub fn transfer_assets_from_user_to_vault(&self, amount: u64) -> Result<()> {
        transfer_checked(
            CpiContext::new(
                self.asset_token_program.to_account_info(),
                TransferChecked {
                    from: self.user_asset_vault.to_account_info(),
                    to: self.asset_vault.to_account_info(),
                    mint: self.asset_mint.to_account_info(),
                    authority: self.user.to_account_info(),
                },
            ),
            amount,
            self.asset_mint.decimals,
        )?;
        Ok(())
    }
    pub fn mint_shares_to_user(&self, bump: u8, amount: u64) -> Result<()> {
        let seeds = &[SHARES_MINT_SEED.as_bytes(), &[bump]];
        let signer_seeds = &[&seeds[..]];
        mint_to_checked(
            CpiContext::new_with_signer(
                self.token2022_program.to_account_info(),
                MintToChecked {
                    mint: self.shares_mint.to_account_info(),
                    authority: self.shares_mint.to_account_info(),
                    to: self.user_shares_vault.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
            self.shares_mint.decimals,
        )?;
        Ok(())
    }
}
