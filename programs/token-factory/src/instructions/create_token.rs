use anchor_lang::prelude::*;

use crate::states::{Access, Config};
use crate::utils::{account::update_account_lamports_to_minimum_balance, ACCESS_SEED, CONFIG_SEED};
use anchor_spl::token_interface::{
    token_metadata_initialize, Mint, TokenInterface,
    TokenMetadataInitialize,
};

#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(seeds = [CONFIG_SEED.as_bytes()], bump, has_one = fee_recipient)]
    pub config: AccountLoader<'info, Config>,
    /// CHECK: This is the account to receive the fee
    #[account(mut)]
    pub fee_recipient: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [ACCESS_SEED.as_bytes(), user.key().as_ref()],
        bump,
    )]
    pub access: Option<AccountLoader<'info, Access>>,
    #[account(
        init, 
        payer = user,
        mint::decimals = 9,
        mint::authority = user.key(),
        mint::token_program = token_program,
        extensions::metadata_pointer::authority = mint.key(),
        extensions::metadata_pointer::metadata_address = mint.key(),
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn process_create_token(ctx: Context<CreateToken>, collect_fee: bool, args: TokenMetadataArgs) -> Result<()> {
    if collect_fee {
        ctx.accounts.collect_fee()?;
    };

    ctx.accounts.initialize_token_metadata(args)?;

    update_account_lamports_to_minimum_balance(
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.rent.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    )?;
    Ok(())
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct TokenMetadataArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

impl<'info> CreateToken<'info> {
    pub fn collect_fee(&self) -> Result<()> {
        let instruction = anchor_lang::solana_program::system_instruction::transfer(
            &self.user.key(),
            &self.config.load()?.fee_recipient,
            self.config.load()?.creation_fee,
        );
        anchor_lang::solana_program::program::invoke(
            &instruction,
            &[
                self.user.to_account_info(),
                self.fee_recipient.to_account_info(),
            ],
        )?;
        Ok(())
    }

    pub fn initialize_token_metadata(&self, args: TokenMetadataArgs) -> Result<()> {
        token_metadata_initialize(
            CpiContext::new(
                self.token_program.to_account_info(),
                TokenMetadataInitialize {
                    program_id: self.token_program.to_account_info(),
                    mint: self.mint.to_account_info(),
                    metadata: self.mint.to_account_info(),
                    mint_authority: self.user.to_account_info(),
                    update_authority: self.user.to_account_info(),
                },
            ),
            args.name,
            args.symbol,
            args.uri,
        )?;
        Ok(())
    }
}
