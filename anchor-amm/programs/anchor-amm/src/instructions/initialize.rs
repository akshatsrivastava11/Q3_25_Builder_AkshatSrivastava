use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::{associated_token::AssociatedToken, token::{transfer_checked, Token, TransferChecked}, token_interface::TokenInterface,token_interface::Mint,token_interface::TokenAccount};
// use anchor_spl::token_interface::Min;

use crate::Config;

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Initialize<'info>{
    #[account(mut)]
    pub initializer:Signer<'info>,


    #[account(
        mint::token_program=token_program
    )]
    pub mintX:InterfaceAccount<'info,Mint>,
    #[account(
        mint::token_program=token_program
    )]
    pub mintY:InterfaceAccount<'info,Mint>,


    #[account(
        init,
        payer=initializer,
        space=8+Config::INIT_SPACE,
        seeds=[b"config",seed.to_le_bytes().as_ref()],
        bump
    )]
    pub  config:Account<'info,Config>,
    #[account(
        mut,
        seeds=[b"lp",config.key().as_ref()],
        bump,
        mint::token_program=token_program,
        mint::authority=config,
        mint::decimals=6
    )]
    pub mint_lp:InterfaceAccount<'info,Mint>,

    #[account(
        init,
        payer=initializer,
        associated_token::mint=mintX,
        associated_token::authority=config
    )]
    pub vault_x:InterfaceAccount<'info,TokenAccount>,
    #[account(
        init,
        payer=initializer,
        associated_token::mint=mintX,
        associated_token::authority=config
    )]
    pub vault_y:InterfaceAccount<'info,TokenAccount>,

    pub token_program:Interface<'info,TokenInterface>,
    pub system_program:Program<'info,System>,
    pub associated_token_program:Program<'info,AssociatedToken>

}


impl<'info>Initialize<'info>{
    pub fn  initialize(&mut self,seed:u64,fees:u16,bumps:InitializeBumps,authority:Option<Pubkey>)->Result<()>{
        self.config.set_inner(Config { 
            mint_x:self.mintX.key() 
            , mint_y: self.mintY.key()
            , authority
            , seed: seed
            , bump_config: bumps.config
            , bump_lp: bumps.mint_lp
            , locked: false
            , fees: fees
         });
         Ok(())
    }
}