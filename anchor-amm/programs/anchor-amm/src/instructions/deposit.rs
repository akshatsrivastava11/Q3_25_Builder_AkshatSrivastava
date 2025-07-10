use anchor_lang::{prelude::*, solana_program::secp256k1_recover::Secp256k1RecoverError, system_program::{transfer, Transfer}};
use anchor_spl::{associated_token::AssociatedToken, token::{mint_to, transfer_checked, MintTo, Token, TransferChecked}, token_interface::{Mint, TokenAccount, TokenInterface}};
use constant_product_curve::ConstantProduct;
// use anchor_spl::token_interface::Min;

use crate::Config;

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Deposit<'info>{
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
        seeds=[b"config",config.seed.to_le_bytes().as_ref()],
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

impl<'info>Deposit<'info>{
    pub fn deposit(&mut self,amount:u64,max_x:u64,max_y:u64)->Result<()>{
        assert!(amount!=0);
        let (x,y)=match self.mint_lp.supply==0 && self.vault_x.amount==0{
            true=>(max_x,max_y),
            false=>{
                let amounts=ConstantProduct::xy_deposit_amounts_from_l(
                    self.vault_x.amount,
                     self.vault_y.amount,
                      self.mint_lp.supply,
                       amount,
                        6)
                        .unwrap();
                        (amounts.x,amounts.y)
            }
        };
        assert!(x<=max_x&&y<=max_y);
        self.deposit_token(true, x)?;
        self.deposit_token(false, y)?;
        self.mint_lp(amount)?;
        Ok(())
    }
    pub fn deposit_token(&mut self,is_x:bool,amount:u64)->Result<()>{
        let cpi_program=self.token_program.to_account_info();
        let[from,to]=match is_x {
            true=>[self.user_x.to_account_info(),self.vault_x.to_account_info()],
            false=>[self.user_y.to_account_info(),self.vault_y.to_account_info()]
        };
        let cpi_accounts=Transfer{
            from,
            to,
        };
        let ctx=CpiContext::new(cpi_program,cpi_accounts);
        transfer(ctx,amount)
    }
    pub fn mint_lp(&mut self,amount:u64)->Result<()>{
        let cpi_program=self.token_program.to_account_info();
        let seeds=&[
            &b"config"[..],
            &self.config.seed.to_le_bytes(),
            &[self.config.bump_lp]
        ];
        let signer_seeds=&[&seeds[..]];

        let mintAccounts=MintTo{mint:self.mint_lp.to_account_info(),authority:self.config.to_account_info(),to:self.user_lp.to_account_info()};
        
        let ctx=CpiContext::new_with_signer(cpi_program,mintAccounts, signer_seeds);
        mint_to(ctx, amount)
    }
}