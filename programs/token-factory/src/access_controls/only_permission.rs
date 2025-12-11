use anchor_lang::prelude::*;

use crate::error::ErrorCode;
use crate::states::Access;

pub fn only_permission(access: &Option<AccountLoader<Access>>, permission: u128) -> Result<()> {
    if let Some(access) = access {
        access.load()?.check_permission(permission)?;
        Ok(())
    } else {
        Err(ErrorCode::UnauthorizedPermission.into())
    }
}
