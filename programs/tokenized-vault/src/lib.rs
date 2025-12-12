use anchor_lang::prelude::*;
use core as core_;

mod access_controls;
mod error;
mod instructions;
mod states;
mod utils;
mod views;

use access_controls::*;
use instructions::*;
use views::*;

declare_id!("6DG8Q5KBjC8UipDajgikmDR6pM8nAtPLzctFpgCUDXbM");

#[program]
pub mod tokenized_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, metadata: TokenMetadataArgs) -> Result<()> {
        process_initialize(ctx, metadata)
    }

    /// =====================================================================================
    /// User Instructions
    /// =====================================================================================

    pub fn deposit(ctx: Context<Deposit>, assets: u64) -> Result<()> {
        process_deposit(ctx, assets)
    }

    pub fn mint(ctx: Context<Deposit>, shares: u64) -> Result<()> {
        process_mint(ctx, shares)
    }

    pub fn withdraw(ctx: Context<Withdraw>, assets: u64) -> Result<()> {
        process_withdraw(ctx, assets)
    }

    pub fn redeem(ctx: Context<Withdraw>, shares: u64) -> Result<()> {
        process_redeem(ctx, shares)
    }

    /// =====================================================================================
    /// Views
    /// =====================================================================================

    pub fn preview_deposit(ctx: Context<Preview>, assets: u64) -> Result<u64> {
        process_preview_deposit(ctx, assets)
    }

    pub fn preview_mint(ctx: Context<Preview>, shares: u64) -> Result<u64> {
        process_preview_mint(ctx, shares)
    }

    pub fn preview_withdraw(ctx: Context<Preview>, assets: u64) -> Result<u64> {
        process_preview_withdraw(ctx, assets)
    }

    pub fn preview_redeem(ctx: Context<Preview>, shares: u64) -> Result<u64> {
        process_preview_redeem(ctx, shares)
    }

    /// =====================================================================================
    /// Admin Manager
    /// =====================================================================================

    #[access_control(only_permission_admin(&ctx.accounts.admin_access, permission))]
    pub fn grant_permission(ctx: Context<UpdatePermission>, permission: u128) -> Result<()> {
        process_grant_permission(ctx, permission)
    }

    #[access_control(only_permission_admin(&ctx.accounts.admin_access, permission))]
    pub fn revoke_permission(ctx: Context<UpdatePermission>, permission: u128) -> Result<()> {
        process_revoke_permission(ctx, permission)
    }

    #[access_control(only_owner(&ctx.accounts.config, *ctx.accounts.owner.key))]
    pub fn grant_permission_admin(
        ctx: Context<UpdatePermissionAdmin>,
        permission: u128,
    ) -> Result<()> {
        process_grant_permission_admin(ctx, permission)
    }

    #[access_control(only_owner(&ctx.accounts.config, *ctx.accounts.owner.key))]
    pub fn revoke_permission_admin(
        ctx: Context<UpdatePermissionAdmin>,
        permission: u128,
    ) -> Result<()> {
        process_revoke_permission_admin(ctx, permission)
    }
}
