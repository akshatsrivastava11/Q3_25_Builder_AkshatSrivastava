use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{close_account, transfer_checked, CloseAccount, TransferChecked}, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::Escrow;


#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Refund<'info>{
    #[account(mut)]
    pub maker:Signer<'info>,

    //mints
    pub mint_x:InterfaceAccount<'info,Mint>,

    //escrow state
    #[account(  
        seeds=[b"escrow",seed.to_le_bytes().as_ref(),maker.key().as_ref()],
        bump,
        
    )]
    pub escrow:Account<'info,Escrow>,

    //vault for putting in mint_x tokens
    #[account(
        mut,
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
    pub maker_x:InterfaceAccount<'info,TokenAccount>,

    //programs
    pub system_program:Program<'info,System>,
    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>
}

impl<'info>Refund<'info>{
    pub fn transfer(&mut self)->Result<()>{
        let cpi_program=self.token_program.to_account_info();
        let accounts=TransferChecked{
            from:self.vault_x.to_account_info(),
            to:self.maker_x.to_account_info(),
            authority:self.escrow.to_account_info(),
            mint:self.mint_x.to_account_info()
        };
        let maker_key=self.maker.key();
        let seeds=&[
            b"escrow",
            &self.escrow.seed.to_le_bytes()[..],
            maker_key.as_ref(),
            &[self.escrow.escrow_bump]
        ];
        let signer_seeds=&[&seeds[..]];
        let ctx=CpiContext::new_with_signer(cpi_program,accounts,signer_seeds);
        transfer_checked(ctx, self.vault_x.amount, self.mint_x.decimals)
    }
    pub fn close(&mut self)->Result<()>{
        let cpi_program=self.token_program.to_account_info();
        let accounts=CloseAccount{
            account:self.vault_x.to_account_info(),
            authority:self.escrow.to_account_info(),
            destination:self.maker.to_account_info()
        };

                let maker_key=self.maker.key();

           let seeds=&[
            b"escrow",
            &self.escrow.seed.to_le_bytes()[..],
            maker_key.as_ref(),
            &[self.escrow.escrow_bump]
        ];

        let signer_seeds=&[&seeds[..]];
        let ctx=CpiContext::new_with_signer(cpi_program, accounts, signer_seeds);
        close_account(ctx)
    }
}