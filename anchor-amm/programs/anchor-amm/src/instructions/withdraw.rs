use std::env::consts::FAMILY;

use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::{associated_token::AssociatedToken, token::{burn, transfer_checked, Burn, Token, TransferChecked}, token_interface::{Mint, TokenAccount, TokenInterface}};
// use anchor_spl::token_interface::Min;

use crate::Config;

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Withdraw<'info>{
     #[account(mut)]
    pub user:Signer<'info>,

    #[account(
        mint::token_program=token_program
    )]
    pub mint_x:InterfaceAccount<'info,Mint>,
    #[account(
        mint::token_program=token_program
    )]
    pub mint_y:InterfaceAccount<'info,Mint>,

    #[account(
        seeds=[b"config",seed.to_le_bytes().as_ref()],
        bump=config.bump_config
    )]
    pub config:Account<'info,Config>,

    #[account(
        mut,
        seeds=[b"lp",config.key().as_ref()],
        bump=config.bump_lp,
        mint::token_program=token_program,
        mint::authority=config,
        mint::decimals=6        
    )]
    pub mint_lp:InterfaceAccount<'info,Mint>,


          #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_lp,
        associated_token::authority = user,
    )]
    pub user_lp:InterfaceAccount<'info,TokenAccount>,


    #[account(
        mut,
        associated_token::token_program=token_program,
        associated_token::mint=mint_x,
        associated_token::authority=config
    )]
    pub vault_x:InterfaceAccount<'info,TokenAccount>,
    #[account(
        mut,
        associated_token::token_program=token_program,
        associated_token::mint=mint_x,
        associated_token::authority=config
    )]
    pub vault_y:InterfaceAccount<'info,TokenAccount>,


    #[account(mut,
        associated_token::mint = mint_x,
        associated_token::authority = user,
    )]
    pub user_x: InterfaceAccount<'info, TokenAccount>,//Associated Token Accounts (ATA). Holds user’s Token X
    
    #[account(mut,
        associated_token::mint = mint_y,
        associated_token::authority = user,
    )]
    pub user_y: InterfaceAccount<'info, TokenAccount>,//Associated Token Accounts (ATA). Holds user’s Token Y




    pub system_program:Program<'info,System>,
    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>
}

impl<'info>Withdraw<'info>{
    pub fn withdraw()->Result<()>{
        todo!()
    }
    pub fn withdraw_token(&mut self,is_x:bool,amount:u64)->Result<()>{
        let (from,to)=match is_x {
                true=>(
                    self.vault_x.to_account_info(),
                    self.user_x.to_account_info()
                ),
                false=>(
                    self.vault_y.to_account_info(),
                    self.user_y.to_account_info()
                )
        };
        let cpi_program=self.token_program.to_account_info();
        let cpi_account=Transfer{
            from,to
        };
    let seeds = &[
            &b"config"[..],
            &self.config.seed.to_le_bytes(),
            &[self.config.bump_config],
        ];
        let signer_seed=&[&seeds[..]];
        let ctx=CpiContext::new_with_signer(cpi_program, cpi_account, signer_seed);
        transfer(ctx, amount)

    }

    pub fn burn_lp_token(&mut self,amount:u64)->Result<()>{
        let cpi_program=self.token_program.to_account_info();
        let cpi_accounts=Burn{
            authority:self.user.to_account_info(),
            from:self.user_lp.to_account_info(),
            mint:self.mint_lp.to_account_info()
        };
        let ctx=CpiContext::new(cpi_program, cpi_accounts);
        burn(ctx, amount)
    }
}