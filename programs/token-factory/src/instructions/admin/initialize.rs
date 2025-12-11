use anchor_lang::prelude::*;

use crate::states::{Access, Config, ConfigInitialized};
use crate::utils::{ACCESS_SEED, CONFIG_SEED};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(init, payer = owner, space = 8 + Access::INIT_SPACE, seeds = [ACCESS_SEED.as_bytes(), owner.key().as_ref()], bump)]
    pub access: AccountLoader<'info, Access>,
    #[account(init, payer = owner, space = 8 + Config::INIT_SPACE, seeds = [CONFIG_SEED.as_bytes()], bump)]
    pub config: AccountLoader<'info, Config>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn process_initialize(ctx: Context<Initialize>) -> Result<()> {
    let config = &mut ctx.accounts.config.load_init()?;
    let access = &mut ctx.accounts.access.load_init()?;

    access.initialize(ctx.accounts.owner.key())?;
    access.grant_permission(u128::MAX)?;
    access.grant_admin_permission(u128::MAX)?;

    config.owner = ctx.accounts.owner.key();
    config.fee_recipient = ctx.accounts.owner.key();
    config.creation_fee = 100000000; // 0.1 SOL
    emit!(ConfigInitialized {
        owner: ctx.accounts.owner.key(),
        fee_recipient: ctx.accounts.owner.key(),
        creation_fee: 100000000,
    });

    Ok(())
}
