

use anchor_lang::prelude::*;
use anchor_spl::{metadata::{freeze_delegated_account, Metadata,MetadataAccount,mpl_token_metadata::{instructions::{FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts}}, FreezeDelegatedAccount, MasterEditionAccount}, token::{approve, Approve, TokenAccount}};

use crate::StakeAccount;
use crate::StakeConfig;
use crate::UserConfig;
#[derive(Accounts)]
pub struct Stake<'info>{
    #[account(mut)]
    pub user:Signer<'info>,
    pub mint:Account<'info,Mint>,
    pub collection_mint:Account<'info,Mint>,
    #[account(
        mut,
        associated_token::authority=user,
        associated_token::mint=mint,

    )]
    pub ata_mint:Account<'info,TokenAccount>,
    #[account(
        seeds=[b"metadata",metadata_program.key().as_ref(),mint.key().as_ref()],
        bump,
        seeds::program=metadata_program.key(),
        constraint=metadata.collection.unwrap().key.as_ref()==collection_mint.key().as_ref(),
        constraint=metadata.collection.as_ref().unwrap().verified==true
    )]
    pub metadata:Account<'info,MetadataAccount>,
    #[account(
        seeds=[b"metadata",metadata_program.key().as_ref(),mint.key().as_ref(),b"edition"],
        bump,
        seeds::program=metadata_program.key()
  )]
    pub master_edition:Account<'info,MasterEditionAccount>,
    #[account(
        init,
        payer=user,
        space=8+StakeAccount::INIT_SPACE,
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

impl<'info>Stake<'info>{
    pub fn stake(&mut self,bumps:StakeBumps)->Result<()>{

        //says the solana program that `allow stake_Account to do anything with 1 of the nft`
     let program=self.token_program.to_account_info();
     let account=Approve{
        authority:self.user.to_account_info(),
        delegate:self.stake_account.to_account_info(),
        to:self.ata_mint.to_account_info()
     };
     let ctx=CpiContext::new(program, account);
     approve(ctx, 1);
     //now the stake account will be freezing over the nft

     let seeds=&[
        b"stake".as_ref(),
        self.user.key().as_ref(),
        self.config.key().as_ref()
     ];
     let signer_seeds=&[&seeds[..]];
     let accounts=FreezeDelegatedAccountCpiAccounts{
        delegate:&self.stake_account.to_account_info(),
        edition:&self.master_edition.to_account_info(),
    
        mint:&self.mint.to_account_info(),
        token_account:&self.ata_mint.to_account_info(),
        token_program:&self.token_program.to_account_info()
     };
     FreezeDelegatedAccountCpi::new(&self.metadata_program.to_account_info(), accounts).invoke_signed(signers_seeds);
     self.stake_account.set_inner(StakeAccount { 
        mint: self.mint.key(),
         owner: self.user.key(), 
         staked_at: Clock::get()?.unix_timestamp,
          bump: bumps.stake_account
         });
         self.user_config.points+=self.config.points_per_stake as u32;
    self.user_config.amount_staked+=1;
    
         Ok(())
    
    //  let ctx=CpiContext::new(program, accounts);
    //  freeze_delegated_account(ctx)


    }
}