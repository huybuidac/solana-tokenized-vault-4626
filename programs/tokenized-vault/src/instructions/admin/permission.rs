use anchor_lang::prelude::*;

use crate::states::Access;
use crate::utils::ACCESS_SEED;

#[derive(Accounts)]
pub struct UpdatePermission<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init_if_needed, 
        payer = admin, 
        space = 8 + Access::INIT_SPACE, 
        seeds = [ACCESS_SEED.as_bytes(), admin.key().as_ref()], 
        bump
    )]
    pub admin_access: AccountLoader<'info, Access>,
    /// CHECK: This is the account to grant the permission to
    pub to: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = admin,
        space = 8 + Access::INIT_SPACE,
        seeds = [ACCESS_SEED.as_bytes(), to.key().as_ref()],
        bump
    )]
    pub to_access: AccountLoader<'info, Access>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn process_grant_permission(ctx: Context<UpdatePermission>, permission: u128) -> Result<()> {
    let mut to_access = match ctx.accounts.to_access.load_mut() {
        Ok(r) => r,
        Err(_err) => {
            let mut access = ctx.accounts.to_access.load_init()?;
            access.initialize(ctx.accounts.to.key())?;
            access
        }
    };
    to_access.grant_permission(permission)?;
    Ok(())
}

pub fn process_revoke_permission(ctx: Context<UpdatePermission>, permission: u128) -> Result<()> {
    let mut to_access = match ctx.accounts.to_access.load_mut() {
        Ok(r) => r,
        Err(_err) => {
            let mut access = ctx.accounts.to_access.load_init()?;
            access.initialize(ctx.accounts.to.key())?;
            access
        }
    };
    to_access.revoke_permission(permission)?;
    Ok(())
}
