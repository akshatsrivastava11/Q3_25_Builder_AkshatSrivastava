use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::{associated_token::AssociatedToken, token::{close_account, transfer_checked, CloseAccount, Token, TransferChecked}, token_interface::{Mint, TokenAccount, TokenInterface}};
// use anchor_spl::token_interface::Min;

use crate::{make, Escrow};


#[derive(Accounts)]
pub struct  Take<'info>{
    #[account(
        mut
    )]
    pub taker:Signer<'info>,
    pub maker:SystemAccount<'info>,
    #[account(
        mint::token_program=token_program
    )]
    pub mint_a:InterfaceAccount<'info,Mint>,
    #[account(
        mint::token_program=token_program
    )]
    pub mint_b:InterfaceAccount<'info,Mint>,
    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint=mint_a,
        associated_token::authority=taker,
        associated_token::token_program=token_program
    )]
    pub taker_mint_a_ata:InterfaceAccount<'info,TokenAccount>,
    #[account(
        mut,
               associated_token::mint=mint_b,
        associated_token::authority=taker,
        associated_token::token_program=token_program 
    )]
    pub taker_mint_b_ata:InterfaceAccount<'info,TokenAccount>,
       #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint=mint_b,
        associated_token::authority=maker,
        associated_token::token_program=token_program
    )]
    pub maker_taker_mint_b:InterfaceAccount<'info,TokenAccount>,
        #[account(
        mut,
        close = maker,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub system_program:Program<'info,System>,
    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>

}

impl<'info>Take<'info>{
    pub fn take(&mut self)->Result<()>{
        //transfer mint_b from taker's mint_b_ata to maker_mint_b_ata
            let cpi_program: AccountInfo<'_>=self.token_program.to_account_info();
        let cpi_Accounts=TransferChecked{from:self.taker_mint_b_ata.to_account_info(),to:self.maker_taker_mint_b.to_account_info(),
        authority:self.taker.to_account_info(),
        mint:self.mint_b.to_account_info()
        };
        let cpi_context=CpiContext::new(cpi_program, cpi_Accounts);
        transfer_checked(cpi_context,self.escrow.amount,self.mint_b.decimals);

        //transfer mint_a from vault to mint_a_ata of taker
        let transferAccounts=TransferChecked{
            from:self.vault.to_account_info(),
            to:self.taker_mint_a_ata.to_account_info(),
            authority:self.escrow.to_account_info(),
            mint:self.mint_a.to_account_info()
        };
             let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump],
        ]];
                    let cpi_program: AccountInfo<'_>=self.token_program.to_account_info();

        let ctx=CpiContext::new_with_signer(cpi_program, transferAccounts, &signer_seeds);
        transfer_checked(ctx, self.vault.amount, self.mint_a.decimals);
        let closeAccount=CloseAccount{
            account:self.vault.to_account_info(),
            destination:self.maker.to_account_info(),
            authority:self.escrow.to_account_info()
        };
        let closeCtx=CpiContext::new_with_signer(self.token_program.to_account_info(),closeAccount,&signer_seeds);
        close_account(closeCtx)
    }
}