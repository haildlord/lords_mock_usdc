use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface, SetAuthority, set_authority, spl_token_2022::instruction::AuthorityType, TokenAccount, MintTo, mint_to};
use anchor_spl::metadata::{Metadata, CreateMetadataAccountsV3, create_metadata_accounts_v3, update_metadata_accounts_v2, UpdateMetadataAccountsV2};
use anchor_spl::associated_token::AssociatedToken;
pub mod constants;
pub mod mock_usdc_errors;

declare_id!("9aURuK86pik3LVQT3nCEF466CfKcKVmNWiETkGegBjx7");

#[program]
pub mod lords_mock_usdc {
    use super::*;
    use crate::constants::*;
    use crate::mock_usdc_errors::ErrorCode;

    pub fn create_mock_usdc(ctx: Context<CreateMockUSDC>) -> Result<()> {

        let cpi_context = CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),

            CreateMetadataAccountsV3{
                metadata : ctx.accounts.metadata_account.to_account_info(),
                mint : ctx.accounts.mint.to_account_info(),
                mint_authority : ctx.accounts.signer.to_account_info(),
                update_authority : ctx.accounts.signer.to_account_info(),
                payer : ctx.accounts.signer.to_account_info(),
                system_program : ctx.accounts.system_program.to_account_info(),
                rent : ctx.accounts.rent.to_account_info()
            }

        );


        let data_v2 = anchor_spl::metadata::mpl_token_metadata::types::DataV2 {
            name: "Lords USDC".to_string(),
            symbol: "LUSDC".to_string(),
            uri: "https://general-crimson-chimpanzee.myfilebase.com/ipfs/QmWTfSmEFwpxevddnZWR5UzjV1rdjum9BvLqEPwqpbzh5j".to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        create_metadata_accounts_v3(cpi_context, data_v2, true, true, None)?;

        let cpi_context_auth = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            SetAuthority{
                current_authority : ctx.accounts.signer.to_account_info(),
                account_or_mint: ctx.accounts.mint.to_account_info(),
            }
        );

        set_authority(
            cpi_context_auth,
            AuthorityType::MintTokens,
            Some(ctx.accounts.mint_authority_pda.key()),
        )?;

        let phantom_wallet = pubkey!("HpAYk14jYpomivS4F7oXySN81sdoPvTaHtFsPgiK2jzf");

        let cpi_ctx_update = CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            UpdateMetadataAccountsV2 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                update_authority: ctx.accounts.signer.to_account_info(),
            },
        );

        update_metadata_accounts_v2(
            cpi_ctx_update,
            Some(phantom_wallet),
            None,
            None,
            None,
        )?;


        Ok(())
    }

    pub fn mint_mock_usdc(ctx: Context<MintMockUSDC>, amount: u64) -> Result<()> {

        if ctx.accounts.signer.key() != DEVNET_ADMIN_PUBKEY {
            require!(
                amount > 0 && amount <= MAX_MOCK_USDC_PER_TX,
                ErrorCode::InvalidMintAmount
            );
        } else {
            require!(amount > 0, ErrorCode::InvalidMintAmount);
        }

        let bump = ctx.bumps.mint_authority_pda;

        let seeds = &[
            b"mint_authority".as_ref(),
            &[bump],
        ];
        let signer_seeds = &[&seeds[..]];


        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.destination.to_account_info(),
                authority: ctx.accounts.mint_authority_pda.to_account_info(),
            },
            signer_seeds,
        );

        mint_to(cpi_context, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateMockUSDC<'info> {

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        mint::decimals = 6,
        mint::authority = signer.key(),
        mint::freeze_authority = pubkey!("HpAYk14jYpomivS4F7oXySN81sdoPvTaHtFsPgiK2jzf")
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    /// CHECK: Protocol PDA that will receive the mint authority
    #[account(seeds = [b"mint_authority"], bump)]
    pub mint_authority_pda: AccountInfo<'info>,

    /// CHECK: Validated by explicit seed checks
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            mint.key().as_ref()
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: AccountInfo<'info>,

    pub token_metadata_program: Program<'info, Metadata>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program : Program<'info, System>,
}



#[derive(Accounts)]
pub struct MintMockUSDC<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        mint::authority = mint_authority_pda,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    /// Anchor will initialize this as an ATA if it doesn't exist
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
    )]
    pub destination: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: The PDA authority that holds the minting power
    #[account(
        seeds = [b"mint_authority"],
        bump,
    )]
    pub mint_authority_pda: AccountInfo<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}