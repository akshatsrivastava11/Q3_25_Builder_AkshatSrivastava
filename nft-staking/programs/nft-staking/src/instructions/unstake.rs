use std::time;

use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            ThawDelegatedAccountCpi, 
            ThawDelegatedAccountCpiAccounts
        }, 
        MasterEditionAccount, 
        Metadata
    }, 
    token::{
        revoke, 
        Mint, 
        Revoke, 
        Token, 
        TokenAccount
    }
};
use crate::error::StakeError;
use crate::{StakeAccount, StakeConfig, UserConfig};

#[derive(Accounts)]
pub struct Unstake<'info>{
    #[account(mut)]
    pub user:Signer<'info>,
     pub mint:Account<'info,Mint>,
    #[account(
        mut,
        associated_token::mint=mint,
        associated_token::authority=user
    )]
    pub ata_mint:Account<'info,TokenAccount>,

    #[account(
        seeds=[b"metadata",metadata_program.key().as_ref(),mint.key().as_ref(),b"edition"],
        bump,
        seeds::program=metadata_program.key()
    )]
    pub master_edition:Account<'info,MasterEditionAccount>,
    #[account(
        mut,
        close=user,
        seeds=[b"stake",stake_config.key().as_ref(),mint.key().as_ref()],
        bump
    )]
    pub stake_account:Account<'info,StakeAccount>,
    #[account(
        seeds=[b"config"],
        bump
    )]
    pub stake_config:Account<'info,StakeConfig>,
    #[account(
        mut,
        seeds=[b"user",user.key().as_ref()],
        bump
    )]
    pub user_config:Account<'info,UserConfig>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub associated_token_program:Program<'info,AssociatedToken>,
    pub metadata_program:Program<'info,Metadata>
}
impl<'info>Unstake<'info>{
    pub fn unstake(&mut self)->Result<()>{
        let time_elapsed=((Clock::get()?.unix_timestamp-self.stake_account.staked_at)/86400) as u32;
        require!(time_elapsed>self.stake_config.freeze_period,StakeError::TimeElapsedError);
        self.user_config.points+=(self.stake_config.points_per_stake as u32 )*time_elapsed;
        
        let program=self.token_program.to_account_info();
        let accounts=ThawDelegatedAccountCpiAccounts{
            mint:&self.mint.to_account_info(),
            delegate:&self.stake_account.to_account_info(),
            edition:&self.master_edition.to_account_info(),
            token_account:&self.ata_mint.to_account_info(),
            token_program:&self.token_program.to_account_info()
        };
      let seeds = &[
            b"stake",
            self.mint.to_account_info().key.as_ref(),
            self.stake_config.to_account_info().key.as_ref(),
            &[self.stake_account.bump]
        ];     
        let signer_seeds = &[&seeds[..]];
        ThawDelegatedAccountCpi::new(&self.metadata_program.to_account_info(), accounts).invoke_signed(signers_seeds);
        let account=Revoke{
            source:self.ata_mint.to_account_info(),
            authority:self.user.to_account_info()
        };
        let ctx=CpiContext::new(program,account);
        revoke(ctx);
        Ok(())
        // todo!()
    }
}
