use anchor_lang::prelude::*;

mod access_controls;
mod error;
mod instructions;
mod states;
mod utils;
mod views;

use access_controls::*;
use instructions::*;
use utils::*;
use views::*;

declare_id!("6DG8Q5KBjC8UipDajgikmDR6pM8nAtPLzctFpgCUDXbM");

#[program]
pub mod token_factory {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        process_initialize(ctx)
    }

    /// =====================================================================================
    /// User Instructions
    /// =====================================================================================
    pub fn create_token(ctx: Context<CreateToken>, args: TokenMetadataArgs) -> Result<()> {
        process_create_token(ctx, true, args)
    }

    #[access_control(only_permission(&ctx.accounts.access, WHITELIST_PERMISSION))]
    pub fn create_token_whitelisted(
        ctx: Context<CreateToken>,
        args: TokenMetadataArgs,
    ) -> Result<()> {
        process_create_token(ctx, false, args)
    }

    /// =====================================================================================
    /// Views
    /// =====================================================================================
    pub fn get_fee(ctx: Context<GetFee>) -> Result<u64> {
        process_get_fee(ctx)
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

    #[access_control(only_owner(&ctx.accounts.config, *ctx.accounts.owner.key))]
    pub fn update_fee_recipient(ctx: Context<UpdateConfig>, fee_recipient: Pubkey) -> Result<()> {
        process_update_fee_recipient(ctx, fee_recipient)
    }

    #[access_control(only_owner(&ctx.accounts.config, *ctx.accounts.owner.key))]
    pub fn update_creation_fee(ctx: Context<UpdateConfig>, creation_fee: u64) -> Result<()> {
        process_update_creation_fee(ctx, creation_fee)
    }
}
