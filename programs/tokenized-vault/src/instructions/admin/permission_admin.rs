use anchor_lang::prelude::*;

use crate::states::{Access, Config};
use crate::utils::{ACCESS_SEED, CONFIG_SEED};

#[derive(Accounts)]
pub struct UpdatePermissionAdmin<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    /// CHECK: This is the account to grant the admin permission to
    pub to: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = owner,
        space = 8 + Access::INIT_SPACE,
        seeds = [ACCESS_SEED.as_bytes(), to.key().as_ref()],
        bump
    )]
    pub to_access: AccountLoader<'info, Access>,
    #[account(seeds = [CONFIG_SEED.as_bytes()], bump)]
    pub config: AccountLoader<'info, Config>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn process_grant_permission_admin(
    ctx: Context<UpdatePermissionAdmin>,
    permission: u128,
) -> Result<()> {
    let mut to_access = match ctx.accounts.to_access.load_mut() {
        Ok(r) => r,
        Err(_err) => {
            let mut access = ctx.accounts.to_access.load_init()?;
            access.initialize(ctx.accounts.to.key())?;
            access
        }
    };
    to_access.grant_admin_permission(permission)?;
    Ok(())
}

pub fn process_revoke_permission_admin(
    ctx: Context<UpdatePermissionAdmin>,
    permission: u128,
) -> Result<()> {
    let mut to_access = match ctx.accounts.to_access.load_mut() {
        Ok(r) => r,
        Err(_err) => {
            let mut access = ctx.accounts.to_access.load_init()?;
            access.initialize(ctx.accounts.to.key())?;
            access
        }
    };
    to_access.revoke_admin_permission(permission)?;
    Ok(())
}
