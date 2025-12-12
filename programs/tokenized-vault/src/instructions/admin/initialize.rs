use anchor_lang::prelude::*;

use crate::states::{Access, Config, ConfigInitialized};
use crate::utils::{
    account::update_account_lamports_to_minimum_balance, ACCESS_SEED, CONFIG_SEED, MAX_DECIMALS,
    SHARES_MINT_SEED,
};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    token_metadata_initialize, Mint, Token2022, TokenAccount, TokenInterface,
    TokenMetadataInitialize,
};

use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(init, payer = owner, space = 8 + Access::INIT_SPACE, seeds = [ACCESS_SEED.as_bytes(), owner.key().as_ref()], bump)]
    pub access: AccountLoader<'info, Access>,
    #[account(init, payer = owner, space = 8 + Config::INIT_SPACE, seeds = [CONFIG_SEED.as_bytes()], bump)]
    pub config: AccountLoader<'info, Config>,

    pub asset_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = owner,
        associated_token::mint = asset_mint,
        associated_token::authority = shares_mint,
        associated_token::token_program = asset_token_program,
    )]
    pub asset_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        seeds = [SHARES_MINT_SEED.as_bytes()],
        bump,
        payer = owner,
        mint::decimals = MAX_DECIMALS,
        mint::authority = shares_mint.key(),
        mint::token_program = token2022_program,
        extensions::metadata_pointer::authority = shares_mint.key(),
        extensions::metadata_pointer::metadata_address = shares_mint,
    )]
    pub shares_mint: InterfaceAccount<'info, Mint>,

    pub asset_token_program: Interface<'info, TokenInterface>,
    pub token2022_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn process_initialize(ctx: Context<Initialize>, metadata: TokenMetadataArgs) -> Result<()> {
    if ctx.accounts.asset_mint.decimals > MAX_DECIMALS {
        return Err(ErrorCode::InvalidAssetMintDecimals.into());
    }

    let config = &mut ctx.accounts.config.load_init()?;
    let access = &mut ctx.accounts.access.load_init()?;

    access.initialize(ctx.accounts.owner.key())?;
    access.grant_permission(u128::MAX)?;
    access.grant_admin_permission(u128::MAX)?;

    config.owner = ctx.accounts.owner.key();
    config.asset_mint = ctx.accounts.asset_mint.key();
    config.decimals_offset = MAX_DECIMALS - ctx.accounts.asset_mint.decimals;

    ctx.accounts
        .initialize_shares_metadata(ctx.bumps.shares_mint, metadata)?;

    update_account_lamports_to_minimum_balance(
        ctx.accounts.shares_mint.to_account_info(),
        ctx.accounts.owner.to_account_info(),
        ctx.accounts.rent.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    )?;

    emit!(ConfigInitialized {
        owner: ctx.accounts.owner.key(),
    });

    Ok(())
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct TokenMetadataArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

impl<'info> Initialize<'info> {
    pub fn initialize_shares_metadata(&self, bump: u8, args: TokenMetadataArgs) -> Result<()> {
        let seeds = &[SHARES_MINT_SEED.as_bytes(), &[bump]];
        let signer = &[&seeds[..]];
        token_metadata_initialize(
            CpiContext::new_with_signer(
                self.token2022_program.to_account_info(),
                TokenMetadataInitialize {
                    program_id: self.token2022_program.to_account_info(),
                    mint: self.shares_mint.to_account_info(),
                    metadata: self.shares_mint.to_account_info(),
                    mint_authority: self.shares_mint.to_account_info(),
                    update_authority: self.shares_mint.to_account_info(),
                },
                signer,
            ),
            args.name,
            args.symbol,
            args.uri,
        )?;
        Ok(())
    }
}
