use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{transfer_checked, TransferChecked}, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::Escrow;

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Make<'info>{
    #[account(mut)]
    pub maker:Signer<'info>,

    //mints
    pub mint_x:InterfaceAccount<'info,Mint>,
    pub mint_y:InterfaceAccount<'info,Mint>,

    //escrow state
    #[account(
        init,
        payer=maker,
        seeds=[b"escrow",seed.to_le_bytes().as_ref(),maker.key().as_ref()],
        space=8+Escrow::INIT_SPACE,
        bump,
        
    )]
    pub escrow:Account<'info,Escrow>,

    //vault for putting in mint_x tokens
    #[account(
        init,
        payer=maker,
        associated_token::mint=mint_x,
        associated_token::authority=escrow,
        associated_token::token_program=token_program
    )]
    pub vault_x:InterfaceAccount<'info,TokenAccount>,

    //user's x token's ata 
    #[account(
        mut,
        associated_token::mint=mint_x,
        associated_token::authority=maker
    )]
    pub maker_y:InterfaceAccount<'info,TokenAccount>,
        #[account(
        mut,
        associated_token::mint=mint_y,
        associated_token::authority=maker
    )]
    pub maker_x:InterfaceAccount<'info,TokenAccount>,

    //programs
    pub system_program:Program<'info,System>,
    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>
}

impl<'info>Make<'info>{
    //this amount is amount to token_y we need to recieve
    pub fn initilize(&mut self,amount_to_be_recieved:u64,seed:u64,bumps:MakeBumps)->Result<()>{
        self.escrow.set_inner(Escrow {
             maker: self.maker.key(), 
            mint_x: self.mint_x.key(),
             mint_y: self.mint_y.key(),
              amonunt: amount_to_be_recieved,
               escrow_bump:bumps.escrow,
                 seed,
                 });
        Ok(())

    }
    //this amount is the amount to token_x for which the amount to token_y is to be received
    pub fn make(&mut self,amount_to_be_deposited:u64)->Result<()>{
        let cpi_program=self.token_program.to_account_info();
        let accounts=TransferChecked{
            from:self.maker_x.to_account_info(),
            to:self.vault_x.to_account_info(),
            mint:self.mint_x.to_account_info(),
            authority:self.maker.to_account_info()
        };
        let ctx=CpiContext::new(cpi_program,accounts);
        transfer_checked(ctx, amount_to_be_deposited, self.mint_x.decimals)
    }
}