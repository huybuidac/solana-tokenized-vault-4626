use anchor_lang::prelude::*;

use crate::states::Access;

pub fn only_permission_admin(access: &AccountLoader<Access>, permission: u128) -> Result<()> {
    access.load()?.check_admin_permission(permission)?;
    Ok(())
}
