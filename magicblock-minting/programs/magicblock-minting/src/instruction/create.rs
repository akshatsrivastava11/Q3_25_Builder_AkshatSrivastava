use anchor_lang::prelude::*;
use anchor_spl::{metadata::{create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3, Metadata, MetadataAccount}, token::{Mint, Token}};

#[derive(Accounts)]
pub struct Create<'info>{
    #[account(mut)]
    pub signer:Signer<'info>,
    #[account(
        init,
        payer=signer,
        seeds=[b"mint"],
        bump,
        mint::decimals=6,
        mint::authority=mint.key(),
        mint::freeze_authority=mint.key()
    )]
    pub mint:Account<'info,Mint>,
    pub metadata:Account<'info,MetadataAccount>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub metadata_program:Program<'info,Metadata>,
    pub rent:Sysvar<'info,Rent>   
}

impl <'info>Create<'info> {
    pub fn create(&mut self,  name:String,
            symbol:String,
            uri:String)->Result<()>{
        let data=DataV2{
            collection:None,
            creators:None,
            name:name,
            symbol:symbol,
            uri:uri,
            seller_fee_basis_points:0,
            uses:None
        };
        let program=self.metadata_program.to_account_info();
        let accounts=CreateMetadataAccountsV3{
            metadata:self.metadata.to_account_info(),
            mint:self.mint.to_account_info(),
            mint_authority:self.mint.to_account_info(),
            update_authority:self.mint.to_account_info(),
            payer:self.signer.to_account_info(),
            system_program:self.system_program.to_account_info(),
            rent:self.rent.to_account_info()
        };

        let ctx=CpiContext::new(program, accounts);
        create_metadata_accounts_v3(ctx, data, true, false, None);
        Ok(())
    }
}