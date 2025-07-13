use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::{associated_token::AssociatedToken, token::{mint_to, MintTo, TransferChecked}, token_interface::{Mint, TokenAccount, TokenInterface}};
use constant_product_curve::ConstantProduct;
use crate::Config;

#[derive(Accounts)]
pub struct Deposit<'info>{
    #[account(mut)]
    pub depositer:Signer<'info>,
    
    //mints
    #[account(
        mint::token_program=token_program
    )]
    pub mint_x:InterfaceAccount<'info,Mint>,
    #[account(
        mint::token_program=token_program
    )]
    pub mint_y:InterfaceAccount<'info,Mint>,

    //state
  
    #[account(
    
        seeds=[b"config",config.seed.to_le_bytes().as_ref(),depositer.key().as_ref()],
        bump
    )]
    pub config:Account<'info,Config>,
    #[account(
 
        mint::authority=config,
        mint::decimals=6,
        seeds=[b"lp",config.key().as_ref()],
        bump
    )]    
    pub mint_lp:InterfaceAccount<'info,Mint>,
    #[account(
    
        associated_token::mint=mint_lp,
        associated_token::authority=mint_lp,
        
    )]
    pub depositer_lp:InterfaceAccount<'info,TokenAccount>,
    #[account(
        associated_token::mint=mint_x,
        associated_token::authority=config,
    )]
    pub vault_x:InterfaceAccount<'info,TokenAccount>,
    #[account(
        associated_token::mint=mint_x,
        associated_token::authority=config,
    )]
    pub vault_y:InterfaceAccount<'info,TokenAccount>,

    #[account(
        associated_token::mint=mint_x,
        associated_token::authority=depositer,
    )]
    pub user_x:InterfaceAccount<'info,TokenAccount>,
    #[account(
        associated_token::mint=mint_x,
        associated_token::authority=depositer,
    )]
    pub user_y:InterfaceAccount<'info,TokenAccount>,
    
    pub system_program:Program<'info,System>,
    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>
}

impl<'info>Deposit<'info>{
    pub fn deposit(&mut self,amount:u64,max_x:u64,max_y:u64)->Result<()>{
        let(x,y)=match self.vault_x.amount==0 && self.mint_lp.supply==0{
            true=>(max_x,max_y),
            false=>{
                let constandProduct=ConstantProduct::xy_deposit_amounts_from_l(
                    self.vault_x.amount,
                     self.vault_y.amount,
                      self.mint_lp.supply,
                       amount,
                        6).unwrap();
                        (constandProduct.x,constandProduct.y)
            }
        };
        assert!(x <= max_x && y <= max_y);
        self.deposit_token(true, x);
        self.deposit_token(false, y);
        self.mint_lp_token(amount)

    }
    //deposit tokens from user x and y ata to vault x and y ata
    //mint lp-token for user 
    pub fn deposit_token(&mut self,is_x:bool,amount:u64)->Result<()>{
        let (from,to)=match is_x{
            true=>(self.user_x.to_account_info(),self.vault_x.to_account_info()),
            false=>(self.user_y.to_account_info(),self.vault_y.to_account_info())
        };
        let program=self.token_program.to_account_info();
        let accounts=Transfer{
            from,
            to
        };
        let ctx=CpiContext::new(program, accounts);
        transfer(ctx, amount)
    }
    pub fn mint_lp_token(&mut self,amount:u64)->Result<()>{
        let program=self.token_program.to_account_info();
        let accounts=MintTo{
            mint:self.mint_lp.to_account_info(),
            to:self.depositer.to_account_info(),
            authority:self.config.to_account_info()
        };
        let ctx=CpiContext::new(program, accounts);
        mint_to(ctx, amount)
    }
}