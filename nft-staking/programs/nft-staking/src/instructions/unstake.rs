
use std::time;

use anchor_lang::prelude::*;
use anchor_spl::{metadata::{freeze_delegated_account, mpl_token_metadata::instructions::{FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts}, FreezeDelegatedAccount, MasterEditionAccount, Metadata, MetadataAccount}, token::{approve, thaw_account, Approve, ThawAccount, TokenAccount}};

use crate::StakeAccount;
use crate::StakeConfig;
use crate::UserConfig;
use crate::error::StakeError
#[derive(Accounts)]
pub struct Unstake<'info>{
       #[account(mut)]
    pub user:Signer<'info>,
    pub mint:Account<'info,Mint>,
    #[account(
        mut,
        associated_token::authority=user,
        associated_token::mint=mint,

    )]
    pub ata_mint:Account<'info,TokenAccount>,
    #[account(
        seeds=[b"metadata",metadata_program.key().as_ref(),mint.key().as_ref(),b"edition"],
        bump,
        seeds::program=metadata_program.key()
  )]
    pub master_edition:Account<'info,MasterEditionAccount>,
    #[account(
        seeds=[b"stake".as_ref(),user.key().as_ref(),config.key().as_ref()],
        bump
    )]
    pub stake_account:Account<'info,StakeAccount>,
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
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub metadata_program:Program<'info,Metadata>
}
impl<'info>Unstake<'info>{
    pub fn unstake(&mut self)->Result<()>{
        let time_elapsed=Clock::get()?.unix_timestamp-self.stake_account.staked_at;
        require!(time_elapsed<(self.config.freeze_period as i64),StakeError::TimeElapsedError);
        require!(self.user_config.amount_staked>0,StakeError::NoNFTStakedError);
        self.user_config.points+=time_elapsed as u32*self.config.points_per_stake as u32;
        self.user_config.amount_staked-=1;
        let userKey=self.user.key();
        let selfkey=self.config.key();
    let seeds=&[
        b"stake".as_ref(),
        userKey.as_ref(),
        selfkey.as_ref()
     ];
     let signer_seeds=&[&seeds[..]];
     let accounts=ThawAccount{
        account:self.ata_mint.to_account_info(),
        authority:self.config.to_account_info(),
        mint:self.mint.to_account_info()
     };
     let ctx=CpiContext::new(self.token_program.to_account_info(), accounts);
     thaw_account(ctx);
     
     Ok(())
    }
}