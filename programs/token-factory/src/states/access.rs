use anchor_lang::prelude::*;

use crate::error::ErrorCode;

#[account(zero_copy)]
#[derive(InitSpace)]
pub struct Access {
    pub account: Pubkey,
    pub permissions: u128,
    pub permission_admins: u128,
    pub _padding1: [u8; 8],
    pub _padding2: [u64; 13],
}

impl Access {
    pub fn initialize(&mut self, account: Pubkey) -> Result<()> {
        self.account = account;
        emit!(AccessInitialized { account });
        Ok(())
    }

    pub fn has_permission(&self, permission: u128) -> bool {
        self.permissions & permission == permission
    }

    pub fn has_admin_permission(&self, permission: u128) -> bool {
        self.permission_admins & permission == permission
    }

    pub fn check_permission(&self, permission: u128) -> Result<()> {
        if !self.has_permission(permission) {
            return Err(ErrorCode::UnauthorizedPermission.into());
        }
        Ok(())
    }

    pub fn check_admin_permission(&self, permission: u128) -> Result<()> {
        if !self.has_admin_permission(permission) {
            return Err(ErrorCode::UnauthorizedAdminPermission.into());
        }
        Ok(())
    }

    pub fn grant_permission(&mut self, permission: u128) -> Result<()> {
        self.permissions |= permission;
        emit!(PermissionGranted {
            account: self.account,
            permission: permission,
        });
        Ok(())
    }

    pub fn revoke_permission(&mut self, permission: u128) -> Result<()> {
        self.permissions &= !permission;
        emit!(PermissionRevoked {
            account: self.account,
            permission: permission,
        });
        Ok(())
    }

    pub fn grant_admin_permission(&mut self, permission: u128) -> Result<()> {
        self.permission_admins |= permission;
        emit!(AdminPermissionGranted {
            account: self.account,
            permission: permission,
        });
        Ok(())
    }

    pub fn revoke_admin_permission(&mut self, permission: u128) -> Result<()> {
        self.permission_admins &= !permission;
        emit!(AdminPermissionRevoked {
            account: self.account,
            permission: permission,
        });
        Ok(())
    }
}

#[event]
pub struct AccessInitialized {
    pub account: Pubkey,
}

#[event]
pub struct PermissionGranted {
    pub account: Pubkey,
    pub permission: u128,
}

#[event]
pub struct PermissionRevoked {
    pub account: Pubkey,
    pub permission: u128,
}

#[event]
pub struct AdminPermissionGranted {
    pub account: Pubkey,
    pub permission: u128,
}

#[event]
pub struct AdminPermissionRevoked {
    pub account: Pubkey,
    pub permission: u128,
}
