use anchor_lang::prelude::*;

use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    burn_checked, transfer_checked, BurnChecked, Mint, Token2022, TokenAccount, TokenInterface,
    TransferChecked,
};

use crate::states::Config;
use crate::utils::{
    preview_redeem_internal, preview_withdraw_internal, CONFIG_SEED, SHARES_MINT_SEED,
};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut, mint::token_program = asset_token_program)]
    pub asset_mint: InterfaceAccount<'info, Mint>,
    #[account(init_if_needed, payer = user, associated_token::mint = asset_mint, associated_token::authority = user, associated_token::token_program = asset_token_program)]
    pub user_asset_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(mut, associated_token::mint = asset_mint, associated_token::authority = shares_mint, associated_token::token_program = asset_token_program)]
    pub asset_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(mut, seeds = [SHARES_MINT_SEED.as_bytes()], bump, mint::token_program = token2022_program)]
    pub shares_mint: InterfaceAccount<'info, Mint>,
    #[account(mut, associated_token::mint = shares_mint, associated_token::authority = user, associated_token::token_program = token2022_program)]
    pub user_shares_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(seeds = [CONFIG_SEED.as_bytes()], bump, has_one = asset_mint)]
    pub config: AccountLoader<'info, Config>,

    pub asset_token_program: Interface<'info, TokenInterface>,
    pub token2022_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn process_withdraw(ctx: Context<Withdraw>, assets: u64) -> Result<()> {
    let shares = preview_withdraw_internal(
        assets,
        ctx.accounts.asset_vault.amount,
        ctx.accounts.shares_mint.supply,
        ctx.accounts.config.load()?.decimals_offset,
    )?;
    ctx.accounts
        .transfer_assets_from_vault_to_user(ctx.bumps.shares_mint, assets)?;
    ctx.accounts.burn_shares_from_user(shares)?;
    Ok(())
}

pub fn process_redeem(ctx: Context<Withdraw>, shares: u64) -> Result<()> {
    let assets = preview_redeem_internal(
        shares,
        ctx.accounts.asset_vault.amount,
        ctx.accounts.shares_mint.supply,
        ctx.accounts.config.load()?.decimals_offset,
    )?;
    ctx.accounts
        .transfer_assets_from_vault_to_user(ctx.bumps.shares_mint, assets)?;
    ctx.accounts.burn_shares_from_user(shares)?;
    Ok(())
}

impl<'info> Withdraw<'info> {
    pub fn transfer_assets_from_vault_to_user(&self, bump: u8, amount: u64) -> Result<()> {
        let seeds = &[SHARES_MINT_SEED.as_bytes(), &[bump]];
        let signer_seeds = &[&seeds[..]];
        transfer_checked(
            CpiContext::new_with_signer(
                self.asset_token_program.to_account_info(),
                TransferChecked {
                    from: self.asset_vault.to_account_info(),
                    to: self.user_asset_vault.to_account_info(),
                    mint: self.asset_mint.to_account_info(),
                    authority: self.shares_mint.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
            self.asset_mint.decimals,
        )?;
        Ok(())
    }
    pub fn burn_shares_from_user(&self, amount: u64) -> Result<()> {
        burn_checked(
            CpiContext::new(
                self.token2022_program.to_account_info(),
                BurnChecked {
                    mint: self.shares_mint.to_account_info(),
                    authority: self.user.to_account_info(),
                    from: self.user_shares_vault.to_account_info(),
                },
            ),
            amount,
            self.shares_mint.decimals,
        )?;
        Ok(())
    }
}
