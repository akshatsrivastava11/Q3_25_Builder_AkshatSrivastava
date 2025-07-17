use anchor_lang::{prelude::*, solana_program::system_instruction::SystemError};
use anchor_spl::{associated_token::AssociatedToken, token::{transfer_checked, TransferChecked}, token_interface::{Mint,TokenAccount, TokenInterface}};

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
    pub 