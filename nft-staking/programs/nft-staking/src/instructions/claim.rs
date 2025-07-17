use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{mint_to, MintTo, TokenAccount}};

use crate::{StakeConfig, UserConfig};

#[derive(Accounts)]
pub struct Claim<'info>{
    #[account(mut)]
    pub user:Signer<'info>,
    #[account(
        seeds=[b"config".as_ref()],
        bump=config.config_bump
    )]
    pub config:Account<'info,StakeConfig>,
    #[account(
        seeds=[b"user",user.key().as_ref()],
        bump=user_config.bump
    )]
    pub user_config:Account<'info,UserConfig>,
    #[account(
        mint::authority=config,
        seeds=[b"reward".as_ref(),config.key().as_ref()],
        bump
    )]
    pub rewards:Account<'info,Mint>,
    #[account(
        init_if_needed,
        payer=user,
        associated_token::authority=user,
        associated_token::mint=rewards
    )]
    pub reward_ata:Account<'info,TokenAccount>,
    pub associated_token_program:Program<'info,AssociatedToken>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>
}

impl<'info>Claim<'info>{
    pub fn claim(&mut self)->Result<()>{
        let program=self.token_program.to_account_info();
            let seeds=&[
        b"stake".as_ref(),
        self.user.key().as_ref(),
        self.config.key().as_ref()
     ];
     let signer_seeds=&[&seeds[..]];
     let mintToAccounts=MintTo{
        authority:self.config.to_account_info(),
        mint:self.rewards.to_account_info(),
        to:self.reward_ata.to_account_info()
     };
     let ctx=CpiContext::new(program,mintToAccounts);

     mint_to(ctx, self.user_config.points as u64 *10_u64.pow(self.rewards.decimals as uu64));
     self.user_config.points=0;
     Ok(())
    }
} 