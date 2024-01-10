use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint as AnchorMint, TokenAccount, TokenInterface};
use spl_token_2022::{extension::ExtensionType, instruction::*, state::Mint};

declare_id!("Bt35E9Zm9nMZp6BiEpfyaoG4AvGbWKcrc5mYPeNCL3ZE");

#[program]
pub mod token_extensions_examples {
    use anchor_lang::solana_program::{system_instruction, system_program};

    use super::*;

    pub fn initialize_no_ext(ctx: Context<InitializeNoExtensions>, decimals: u8) -> Result<()> {
        Ok(())
    }

    pub fn initialize_with_ext(ctx: Context<InitializeWithExtensions>, decimals: u8) -> Result<()> {
        let space =
            ExtensionType::try_calculate_account_len::<Mint>(&[ExtensionType::PermanentDelegate])
                .unwrap();
        let rent = Rent::get()?.minimum_balance(space);

        let cpi_accounts = anchor_lang::system_program::CreateAccount {
            from: ctx.accounts.payer.to_account_info(),
            to: ctx.accounts.mint.to_account_info(),
        };

        let seeds = [b"mint-ext".as_ref(), &[ctx.bumps.mint]];
        let signer_seeds = [&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            cpi_accounts,
            &signer_seeds,
        );

        anchor_lang::system_program::create_account(
            cpi_ctx,
            rent,
            space as u64,
            ctx.accounts.token_program.key,
        )?;

        let ix = spl_token_2022::instruction::initialize_permanent_delegate(
            ctx.accounts.token_program.key,
            ctx.accounts.mint.key,
            ctx.accounts.delegate.key,
        )?;

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.delegate.to_account_info(),
            ],
        )?;

        let accounts = anchor_spl::token_interface::InitializeMint2 {
            mint: ctx.accounts.mint.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), accounts);
        anchor_spl::token_interface::initialize_mint2(
            cpi_ctx,
            decimals,
            ctx.accounts.mint_auth.key,
            Some(ctx.accounts.freeze_auth.key),
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(decimals: u8)]
pub struct InitializeNoExtensions<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(seeds = [b"auth"], bump)]
    /// CHECK: PDA AUTH
    pub some_auth: AccountInfo<'info>,

    #[account(init,
        payer = payer,
        mint::decimals = decimals,
        mint::authority = some_auth,
        mint::token_program = token_program,
        seeds = [ b"mint-no-ext"],
        bump
    )]
    pub mint: Box<InterfaceAccount<'info, AnchorMint>>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(decimals: u8)]
pub struct InitializeWithExtensions<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(seeds = [b"auth"], bump)]
    /// CHECK: PDA AUTH
    pub some_auth: AccountInfo<'info>,

    /// CHECK: DELEGATE
    pub delegate: AccountInfo<'info>,

    /// CHECK: DELEGATE
    pub freeze_auth: AccountInfo<'info>,

    /// CHECK: DELEGATE
    pub mint_auth: AccountInfo<'info>,

    #[account(mut,
        seeds = [ b"mint-ext"],
        bump
    )]
    /// CHECK: mint initialisation
    pub mint: UncheckedAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}
