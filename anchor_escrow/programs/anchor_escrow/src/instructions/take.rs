use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{close_account, transfer_checked, CloseAccount, TransferChecked}, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::Escrow;


#[derive(Accounts)]

pub struct Take<'info>{
    #[account(mut)]
    pub taker:Signer<'info>,

    pub maker:SystemAccount<'info>,
    #[account(
        mint::token_program=token_program
    )]
    pub mint_x:InterfaceAccount<'info,Mint>,
    #[account(
        mint::token_program=token_program
    )]
    pub mint_y:InterfaceAccount<'info,Mint>,
    
    #[account(
        seeds=[b"escrow",escrow.seed.to_le_bytes().as_ref(),maker.key().as_ref()],
        bump,
        
    )]
    pub escrow:Account<'info,Escrow>,
        #[account(
        mut,
        associated_token::mint=mint_x,
        associated_token::authority=escrow,
        associated_token::token_program=token_program
    )]
    pub vault_x:InterfaceAccount<'info,TokenAccount>,

    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint=mint_x,
        associated_token::authority=taker,
        associated_token::token_program=token_program
    )]
    pub taker_mint_x:InterfaceAccount<'info,TokenAccount>,
    #[account(
        associated_token::mint=mint_y,
        associated_token::authority=taker,
        associated_token::token_program=token_program
    )]
    pub taker_mint_y:InterfaceAccount<'info,TokenAccount>,

    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint=mint_y,
        associated_token::authority=maker,
        associated_token::token_program=token_program
    )]
    pub maker_mint_y:InterfaceAccount<'info,TokenAccount>,

    pub system_program:Program<'info,System>,
    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>
}

impl<'info>Take<'info>{
    //transfer mint y from taker_mint_y to maker_mint_y
    pub fn transfer_to_maker_mint_y(&mut self)->Result<()>{
          let cpi_program=self.token_program.to_account_info();
        let accounts=TransferChecked{
            from:self.taker_mint_y.to_account_info(),
            to:self.maker_mint_y.to_account_info(),
            authority:self.taker.to_account_info(),
            mint:self.mint_y.to_account_info()
        };
  
        let ctx=CpiContext::new(cpi_program,accounts);
        transfer_checked(ctx, self.vault_x.amount, self.mint_y.decimals)
    }
    //transfer mint_x from vault to taker_mint_x
    pub fn transfer_from_vault_to_taker_mint_x(&mut self)->Result<()>{
            let cpi_program=self.token_program.to_account_info();
        let accounts=TransferChecked{
            from:self.vault_x.to_account_info(),
            to:self.taker_mint_x.to_account_info(),
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
}