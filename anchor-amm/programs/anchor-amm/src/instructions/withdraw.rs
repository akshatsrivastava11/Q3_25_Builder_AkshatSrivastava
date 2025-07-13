use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::{associated_token::AssociatedToken, token::{burn, mint_to, Burn, MintTo, TransferChecked}, token_2022::spl_token_2022::extension::confidential_mint_burn::processor, token_interface::{Mint, TokenAccount, TokenInterface}};
use constant_product_curve::ConstantProduct;
use crate::{withdraw, Config};

#[derive(Accounts)]
pub struct Withdraw<'info>{
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
        init,
        payer=depositer,
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
impl<'info>Withdraw<'info>{
    pub fn withdraw(&mut self,amountLp:u64,max_x:u64,max_y:u64)->Result<()>{
        let (x,y)=match self.vault_x.amount==0 && self.vault_y.amount==0 && self.mint_lp.supply==0{
            true=>(max_x,max_y),
            false=>{
                let constants=ConstantProduct::xy_deposit_amounts_from_l(
                    self.vault_x.amount, 
                    self.vault_y.amount, 
                    self.mint_lp.supply, 
                    amountLp, 
                    6).unwrap();
                    (constants.x,constants.y)
            }
        };
        self.withdraw_to_depositer(false, y);
        self.withdraw_to_depositer(true, x);
        self.burn_lp_token(amountLp)

    }
    //withdraw from the vaultx and y
    //burn the mint_lp token
    pub fn withdraw_to_depositer(&mut self,is_x:bool,amount:u64)->Result<()>{
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
    pub fn burn_lp_token(&mut self,amount:u64)->Result<()>{
        let program=self.token_program.to_account_info();
        let account=Burn{
            authority:self.config.to_account_info(),   
            from:self.depositer_lp.to_account_info(),
            mint:self.mint_lp.to_account_info()
        };
                let key=self.depositer.key();

        let signer=&[
            &b"config"[..],
            &self.config.seed.to_le_bytes(),
            key.as_ref(),
            &[self.config.bump]
        ];
        let signer_seeds=&[&signer[..]];
        let ctx=CpiContext::new_with_signer(program, account, signer_seeds);
        burn(ctx, amount)
    }
}