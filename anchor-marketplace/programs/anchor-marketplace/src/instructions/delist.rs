use anchor_lang::{prelude::*, solana_program::system_instruction::SystemError};
use anchor_spl::{associated_token::AssociatedToken, token::{close_account, transfer_checked, CloseAccount, TransferChecked}, token_interface::{Mint,TokenAccount, TokenInterface}};

use crate::{Listing, Marketplace};


#[derive(Accounts)]
pub struct Delist<'info>{
    #[account(mut)]
    pub maker:Signer<'info>,
    #[account(
        seeds=[b"marketplace",name.as_bytes(),admin.key.as_ref()],
        bump        
    )]
    pub marketplace:Account<'info,Marketplace>,
    #[account(
        seeds=[b"listing",marketplace.key().as_ref()],
        bump
    )]
    pub listing:Account<'info,Listing>,
    pub mint:InterfaceAccount<'info,Mint>,
    #[account(
        mut,
        associated_token::authority=maker,
        associated_token::mint=mint,
    )]
    pub user_mint_ata:InterfaceAccount<'info,TokenAccount>,
    #[account(
        associated_token::authority=listing,
        associated_token::mint=mint,        
    )]
    pub vault_ata:InterfaceAccount<'info,TokenAccount>,
    pub system_program:Program<'info,System>,
    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>
}


//transering the tokens back to the user 
//closing the vault
impl <'info>Delist<'info> {
    pub fn delist(&mut self)->Result<()>{
        self.transfer();
        self.close_vault()
// todo!()
    } 
    pub fn transfer(&mut self)->Result<()>{
        let key=self.marketplace.key();
        let seeds=&[
            b"listing",
            key.as_ref(),
            &[self.listing.bump]
        ];
        let signer_seeeds=&[&seeds[..]];
        let accounts=TransferChecked{
            from:self.vault_ata.to_account_info(),
            authority:self.listing.to_account_info(),
            mint:self.mint.to_account_info(),
            to:self.user_mint_ata.to_account_info()
        };
    
        let program=self.token_program.to_account_info();
        let ctx=CpiContext::new_with_signer(program, accounts, signer_seeds);
        transfer_checked(ctx, 1, 0)
        // todo!()
    
    }
    pub fn close_vault(&mut self)->Result<()>{
        let program=self.token_program.to_account_info();
        let seeds=&[
            b"listing",
            key.as_ref(),
            &[self.listing.bump]
        ];
        let signer_seeeds=&[&seeds[..]];
        let accounts=CloseAccount{
            account:self.vault_ata.to_account_info(),
            authority:self.listing.to_account_info(),
            destination:self.maker.to_account_info()
        };
        let ctx=CpiContext::new_with_signer(program, accounts, signer_seeds);
        close_account(ctx)
        // todo!()
    }

}