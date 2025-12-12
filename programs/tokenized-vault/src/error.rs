use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized owner")]
    UnauthorizedOwner,
    #[msg("Unauthorized permission")]
    UnauthorizedPermission,
    #[msg("Unauthorized admin permission")]
    UnauthorizedAdminPermission,
    #[msg("Invalid asset mint decimals")]
    InvalidAssetMintDecimals,
}
