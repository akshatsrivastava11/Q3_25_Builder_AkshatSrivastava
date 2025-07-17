
use anchor_lang::{prelude::*, solana_program::system_instruction::SystemError};
use anchor_spl::{associated_token::AssociatedToken, metadata::{MasterEditionAccount, Metadata, MetadataAccount}, token::{transfer_checked, TransferChecked}, token_interface::{Mint,TokenAccount, TokenInterface}};

use crate::{Listing, Marketplace};

#[derive(Accounts)]
pub struct List<'info>{
    #[account(mut)]
    pub maker:Signer<'info>,
    #[account(
        seeds=[b"marketplace",name.as_bytes(),admin.key.as_ref()],
        bump        
    )]
    pub marketplace:Account<'info,Marketplace>,
    #[account(
        init,
        payer=maker,
        space=8+Listing::INIT_SPACE,
        seeds=[b"listing",marketplace.key().as_ref()],
        bump
    )]
    pub listing:Account<'info,Listing>,
    pub mint:InterfaceAccount<'info,Mint>,
    #[account(
        mut,
        associated_token::mint=mint,
        associated_token::authority=maker,
    )]
    pub user_mint_ata:InterfaceAccount<'info,TokenAccount>,
    //metadata stuff for the mint
    #[account(
        init,
        payer=maker,
        associated_token::mint=mint,        
        associated_token::authority=listing,
    )]
    pub vault_ata:InterfaceAccount<'info,TokenAccount>,
    pub collection:InterfaceAccount<'info,Mint>,
    #[account(
        seeds=[b"metadata",metadata_program.key().as_ref(),mint.key().as_ref()],
        bump,
        seeds::program=metadata_program.key(),
        constraint=metadata.collection.as_ref().unwrap().key.as_ref()==collection.key().as_ref(),
        constraint=metadata.collection.as_ref().unwrap().verified==true
    )]
    pub metadata:Account<'info,MetadataAccount>,
    #[account(
        seeds=[b"metadata",metadata_program.key().as_ref(),mint.key().as_ref(),b"edition"],
        bump,
        seeds::program=metadata_program.key(),
    )]
    pub master_edition:Account<'info,MasterEditionAccount>,
    pub system_program:Program<'info,System>,
    pub metadata_program:Program<'info,Metadata>,
    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>
}


//create the list
//transfer the nft to vault and 
impl<'info>List<'info>{
    pub fn list(&mut self,price:u64,bumps:ListBumps)->Result<()>{
        self.createList(price, bumps);
        self.transfer_nft()
    }
    //creating the list
    pub fn createList(&mut self,price:u64,bumps:ListBumps)->Result<()>{
        self.listing.set_inner(Listing { 
            maker: self.maker.key(),
             mint: self.mint.key(), 
             price,
              bump:bumps.listing
             });
             Ok(())
    }
    pub fn transfer_nft(&mut self )->Result<()>{
        let program=self.token_program.to_account_info();
        let accounts=TransferChecked{
            from:self.user_mint_ata.to_account_info(),
            to:self.vault_ata.to_account_info(),
            mint:self.mint.to_account_info(),
            authority:self.maker.to_account_info()
        };
        let ctx=CpiContext::new(program, accounts);
        transfer_checked(ctx, 1, 0);
        Ok(())

    }
}