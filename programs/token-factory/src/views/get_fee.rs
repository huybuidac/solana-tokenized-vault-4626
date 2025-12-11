use anchor_lang::prelude::*;

use crate::states::Config;
use crate::utils::CONFIG_SEED;

#[derive(Accounts)]
pub struct GetFee<'info> {
    #[account(seeds = [CONFIG_SEED.as_bytes()], bump)]
    pub config: AccountLoader<'info, Config>,
}

pub fn process_get_fee(ctx: Context<GetFee>) -> Result<u64> {
    Ok(ctx.accounts.config.load()?.creation_fee)
}
