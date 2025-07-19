use anchor_lang::prelude::*;
use anchor_spl::{metadata::{mpl_token_metadata::instructions::{FreezeDelegatedAccount, FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts, ThawDelegatedAccountCpi, ThawDelegatedAccountCpiAccounts}, MasterEditionAccount, Metadata, MetadataAccount, SetAndVerifyCollection}, token::{approve, revoke, Approve, FreezeAccount, Mint, Revoke, Token, TokenAccount} };

use crate::{StakeAccount, StakeConfig, UserConfig};
use crate::error::StakeError;
#[derive(Accounts)]
pub struct Unstake<'info>{
    #[account(mut)]
    pub user:Signer<'info>,
    pub mint:Account<'info,Mint>,
    #[account(
        mut,
        associated_token::mint=mint,
        associated_token::authority=user,
    )]
    pub user_mint_ata:Account<'info,TokenAccount>,
    #[account(
        seeds=[b"metadata",metadata_program.key().as_ref(),mint.key().as_ref(),b"edition"],
        bump,
        seeds::program=metadata_program,
    )]
    pub master_edition:Account<'info,MasterEditionAccount>,
    #[account(
        seeds=[b"user",stake_config.key().as_ref(),user.key().as_ref()],
        bump=user_config.bump,
    )]
    pub user_config:Account<'info,UserConfig>,
    #[account(
        seeds=[b"config"],
        bump=stake_config.stake_config_bump
    )]
    pub stake_config:Account<'info,StakeConfig>,
    #[account(
        mut,
        seeds=[b"stake",user_config.key().as_ref()],
        bump
    )]
    pub stake_account:Account<'info,StakeAccount>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub metadata_program:Program<'info,Metadata>
}   


//defreeze the account(thaw the account)
//revoke the access from the stake_account over the nft 
impl<'info>Unstake<'info>{
    pub fn unstake(&mut self,bumps:UnstakeBumps)->Result<()>{
        let time_elapsed=Clock::get()?.unix_timestamp-self.stake_account.staked_at;
        require!(time_elapsed>=self.stake_config.freeze_period as i64,StakeError::TimeElapsedError);
        require!(self.user_config.amounts_staked>0,StakeError::NoNFTStakedError);
        let program=self.metadata_program.to_account_info();
        let binding = self.user_config.key();
        msg!("bumps for stake account is {}",bumps.stake_account);
        let seeds=&[
            b"stake",binding.as_ref(),
            &[bumps.stake_account]
        ];
        let signers_seeds=&[&seeds[..]];
        let accounts=ThawDelegatedAccountCpiAccounts{
            delegate:&self.stake_account.to_account_info(),
            edition:&self.master_edition.to_account_info(),
            mint:&self.mint.to_account_info(),
            token_account:&self.user_mint_ata.to_account_info(),
            token_program:&self.token_program.to_account_info()
        };
        ThawDelegatedAccountCpi::new(&self.metadata_program.to_account_info(), accounts).invoke_signed(signers_seeds)?;
        let account=Revoke{
            authority:self.user.to_account_info(),
            source:self.user_mint_ata.to_account_info()
        };
        let ctx=CpiContext::new(self.token_program.to_account_info(),account);
        revoke(ctx)?;
        self.user_config.points+=time_elapsed as u64*self.stake_config.points_per_stake;
        self.user_config.amounts_staked-=1;
        Ok(())
    }
}