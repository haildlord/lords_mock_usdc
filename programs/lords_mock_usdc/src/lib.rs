use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface, SetAuthority, set_authority, spl_token_2022::instruction::AuthorityType};
use anchor_spl::metadata::{Metadata, CreateMetadataAccountsV3, create_metadata_accounts_v3};


declare_id!("Wd48Y14eLM4zQmLWyEnZ12whBirZeUSNSTNbNW7HPBa");

#[program]
pub mod lords_mock_usdc {
    use super::*;

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
            uri: "https://general-crimson-chimpanzee.myfilebase.com/ipfs/QmTr7MxHH3eeTfmhAvWdrZKY2BnWEnmc8LWWKAj8QyDmiq".to_string(),
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
        mint::freeze_authority = signer.key(),
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    /// CHECK: Protocol PDA that will receive the mint authority
    #[account(seeds = [b"mint_authority"], bump)]
    pub mint_authority_pda: AccountInfo<'info>,

    /// CHECK: Metaplex will validate this
    #[account(mut)]
    pub metadata_account: AccountInfo<'info>,

    pub token_metadata_program: Program<'info, Metadata>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program : Program<'info, System>,
}
