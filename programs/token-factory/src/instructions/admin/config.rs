use anchor_lang::prelude::*;

use crate::states::{Config, CreationFeeUpdated, FeeRecipientUpdated};
use crate::utils::CONFIG_SEED;

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(seeds = [CONFIG_SEED.as_bytes()], bump)]
    pub config: AccountLoader<'info, Config>,
}

pub fn process_update_fee_recipient(
    ctx: Context<UpdateConfig>,
    fee_recipient: Pubkey,
) -> Result<()> {
    let config = &mut ctx.accounts.config.load_mut()?;
    config.fee_recipient = fee_recipient;
    emit!(FeeRecipientUpdated { fee_recipient });
    Ok(())
}

pub fn process_update_creation_fee(ctx: Context<UpdateConfig>, creation_fee: u64) -> Result<()> {
    let config = &mut ctx.accounts.config.load_mut()?;
    config.creation_fee = creation_fee;
    emit!(CreationFeeUpdated { creation_fee });
    Ok(())
}
