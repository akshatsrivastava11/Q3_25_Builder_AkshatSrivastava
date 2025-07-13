use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::{associated_token::AssociatedToken, token::{mint_to, MintTo, TransferChecked}, token_interface::{Mint, TokenAccount, TokenInterface}};
use constant_product_curve::ConstantProduct;
use crate::Config;


#[derive(Accounts)]
pub struct Swap<'info>{
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

impl<'info>Swap<'info>{
    pub fn swap(&mut self,amount_in:u64,min_out:u64,is_x_in:bool)->Result<()>{
        let x_reserve=self.vault_x.amount;
        let y_reserve=self.vault_y.amount;
      let constantProducts=ConstantProduct::init(
        x_reserve,
         y_reserve,
          self.mint_lp.supply,
        self.config.fees,
            Some(6));
            let amt_out=if is_x_in{
                // let new_x=x_reserve.checked_add(amount_in);
                let new_y=ConstantProduct::y2_from_x_swap_amount(x_reserve, y_reserve, amount_in).unwrap();
                y_reserve.checked_sub(new_y).unwrap()
            }else{
                let new_x=ConstantProduct::x2_from_y_swap_amount(x_reserve, y_reserve, amount_in).unwrap();
                x_reserve.checked_sub(new_x).unwrap()
            };
            self.transfer_in(is_x_in, amount_in);
            self.transfer_out(is_x_in, amt_out)

}
    //deposit your token either x or y in the vault_x,vault_y
    pub fn transfer_in(&mut self,is_x:bool,amount:u64)->Result<()>{
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
    //withdraw the other token with proper lineage
    pub fn transfer_out(&mut self,is_x:bool,amount:u64)->Result<()>{
            let (to,from)=match is_x{
            true=>(self.user_x.to_account_info(),self.vault_x.to_account_info()),
            false=>(self.user_y.to_account_info(),self.vault_y.to_account_info())
        };
        let program=self.token_program.to_account_info();
        let accounts=Transfer{
            from,
            to
        };
        let key=self.depositer.key();
        let signer=&[
            &b"config"[..],
            &self.config.seed.to_le_bytes(),
            key.as_ref(),
            &[self.config.bump]
        ];
        let signer_seeds=&[&signer[..]];
        let ctx=CpiContext::new_with_signer(program, accounts, signer_seeds);
        transfer(ctx, amount)   
    }

}