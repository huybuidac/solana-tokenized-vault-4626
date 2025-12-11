use anchor_lang::prelude::*;

use crate::error::ErrorCode;
use crate::states::Config;

pub fn only_owner(config: &AccountLoader<Config>, account: Pubkey) -> Result<()> {
    if config.load()?.owner != account {
        return Err(ErrorCode::UnauthorizedOwner.into());
    }
    Ok(())
}
