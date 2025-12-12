use anchor_lang::prelude::*;

use crate::utils::{MulDiv, Rounding, U128};

pub fn preview_deposit_internal(
    assets: u64,
    total_assets: u64,
    total_shares: u64,
    decimals_offset: u8,
) -> Result<u64> {
    let shares = convert_to_shares(
        assets,
        total_assets,
        total_shares,
        decimals_offset,
        Rounding::Floor,
    )?;
    Ok(shares)
}

pub fn preview_mint_internal(
    shares: u64,
    total_assets: u64,
    total_shares: u64,
    decimals_offset: u8,
) -> Result<u64> {
    let assets = convert_to_assets(
        shares,
        total_assets,
        total_shares,
        decimals_offset,
        Rounding::Ceiling,
    )?;
    Ok(assets)
}

pub fn preview_withdraw_internal(
    assets: u64,
    total_assets: u64,
    total_shares: u64,
    decimals_offset: u8,
) -> Result<u64> {
    let shares = convert_to_shares(
        assets,
        total_assets,
        total_shares,
        decimals_offset,
        Rounding::Ceiling,
    )?;
    Ok(shares)
}

pub fn preview_redeem_internal(
    shares: u64,
    total_assets: u64,
    total_shares: u64,
    decimals_offset: u8,
) -> Result<u64> {
    let assets = convert_to_assets(
        shares,
        total_assets,
        total_shares,
        decimals_offset,
        Rounding::Floor,
    )?;
    Ok(assets)
}

pub fn convert_to_shares(
    assets: u64,
    total_assets: u64,
    total_shares: u64,
    decimals_offset: u8,
    rounding: Rounding,
) -> Result<u64> {
    let offset = 10u64.pow(decimals_offset as u32);
    msg!(
        "convert_to_shares assets: {}, total_assets: {}, total_shares: {}, decimals_offset: {}, offset: {}",
        assets,
        total_assets,
        total_shares,
        decimals_offset,
        offset,
    );
    let x: u64 = U128::from(assets)
        .mul_div(
            U128::from(total_shares + offset),
            U128::from(total_assets + 1),
            rounding,
        )
        .unwrap()
        .as_u64();
    Ok(x)
}

pub fn convert_to_assets(
    shares: u64,
    total_assets: u64,
    total_shares: u64,
    decimals_offset: u8,
    rounding: Rounding,
) -> Result<u64> {
    let offset = 10u64.pow(decimals_offset as u32);
    let x: u64 = U128::from(shares)
        .mul_div(
            U128::from(total_assets + 1),
            U128::from(total_shares + offset),
            rounding,
        )
        .unwrap()
        .as_u64();
    msg!(
        "convert_to_assets shares: {}, total_assets: {}, total_shares: {}, decimals_offset: {}, offset: {}, x: {}",
        shares,
        total_assets,
        total_shares,
        decimals_offset,
        offset,
        x,
    );
    Ok(x)
}
