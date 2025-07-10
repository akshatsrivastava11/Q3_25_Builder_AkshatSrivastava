use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::{associated_token::AssociatedToken, token::{transfer_checked, Token, TransferChecked}, token_interface::TokenInterface,token_interface::Mint,token_interface::TokenAccount};
// use anchor_spl::token_interface::Min;

use crate::{make, Escrow};

#[derive(Accounts)]
pub  struct Make<'info>{
    #[account(mut)]
    pub maker:Signer<'info>,
   
   
    #[account(
        mint::token_program=token_program
    )
    ]
    pub mint_a:InterfaceAccount<'info,Mint>,
     #[account(
        mint::token_program=token_program
    )
    ]
    pub mint_b:InterfaceAccount<'info,Mint>,
   
   
    #[account(
        mut,
        associated_token::mint=mint_a,
        associated_token::authority=maker,
        associated_token::token_program=token_program
    )]
    pub maker_ata_mint_a:InterfaceAccount<'info,TokenAccount>,
   
   #[account(
    init,
    payer=maker,
    space=8+Escrow::INIT_SPACE,
    seeds=[b"escrow",maker.key().as_ref()],
    bump
   )]
   pub escrow:Account<'info,Escrow>,

   #[account(
    init,
    payer=maker,
    associated_token::mint=mint_a,
    associated_token::authority=escrow,
    associated_token::token_program=token_program
   )]
   pub vault:InterfaceAccount<'info,TokenAccount>,
   
    pub system_program:Program<'info,System>,
    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>
}

impl<'info>Make<'info>{
    pub fn initialize(&mut self,amount:u64,bumps:MakeBumps)->Result<()>{
        self.escrow.set_inner(Escrow { 
            maker: self.maker.key(), 
            mint_a: self.mint_a.key(),
             mint_b: self.mint_b.key(),
              amount: amount,
                bump:bumps.escrow 
             });  
             Ok(())     
    }
    pub fn make_offer(&mut self,amount:u64)->Result<()>{
        let cpi_program=self.token_program.to_account_info();
        let cpi_Accounts=TransferChecked{from:self.maker_ata_mint_a.to_account_info(),to:self.vault.to_account_info(),
        authority:self.maker.to_account_info(),
        mint:self.mint_a.to_account_info()
        };
        let cpi_context=CpiContext::new(cpi_program, cpi_Accounts);
        transfer_checked(cpi_context,amount,self.mint_a.decimals)
    }

}