use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::{associated_token::AssociatedToken, token::{close_account, transfer_checked, CloseAccount, Token, TransferChecked}, token_interface::{Mint, TokenAccount, TokenInterface}};
// use anchor_spl::token_interface::Min;

use crate::{make, Escrow};


#[derive(Accounts)]

pub struct Refund<'info>{
      #[account(mut)]
    pub maker:Signer<'info>,
   
   
    #[account(
        mint::token_program=token_program
    )
    ]
    pub mint_a:InterfaceAccount<'info,Mint>,   
   
    #[account(
        mut,
        associated_token::mint=mint_a,
        associated_token::authority=maker,
        associated_token::token_program=token_program
    )]
    pub maker_ata_mint_a:InterfaceAccount<'info,TokenAccount>,
   
   #[account(
    mut,
    close=maker,
    has_one=mint_a,
    has_one=maker,
    seeds=[b"escrow",maker.key().as_ref()],
    bump=escrow.bump
   )]
   pub escrow:Account<'info,Escrow>,

   #[account(
    mut ,
    associated_token::mint=mint_a,
    associated_token::authority=escrow,
    associated_token::token_program=token_program
   )]
   pub vault:InterfaceAccount<'info,TokenAccount>,
   
    pub system_program:Program<'info,System>,
    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>
}
impl<'info>Refund<'info>{
    pub fn refund_and_close(&mut self)->Result<()>{
          let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &[self.escrow.bump],
        ]];
        let cpi_program_for_refund=self.token_program.to_account_info();
        let cpi_accounts=TransferChecked{authority:self.escrow.to_account_info(),from:self.vault.to_account_info(),mint:self.mint_a.to_account_info(),to:self.maker_ata_mint_a.to_account_info()};
        let cpi_context_refund=CpiContext::new_with_signer(cpi_program_for_refund, cpi_accounts,&signer_seeds);
        transfer_checked(cpi_context_refund, self.vault.amount,self.mint_a.decimals);
        let cpi_program_for_refund=self.token_program.to_account_info();

        let close_Account=CloseAccount{
            authority:self.escrow.to_account_info(),
            account:self.vault.to_account_info(),
            destination:self.maker_ata_mint_a.to_account_info()
            };
            let close_cpi_context=CpiContext::new_with_signer(cpi_program_for_refund, close_Account, &signer_seeds);
            close_account(close_cpi_context)

    }
}