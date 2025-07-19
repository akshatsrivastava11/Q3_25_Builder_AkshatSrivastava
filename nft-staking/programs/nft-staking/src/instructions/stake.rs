use anchor_lang::{prelude::*};
use anchor_spl::{metadata::{mpl_token_metadata::instructions::{FreezeDelegatedAccount, FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts}, MasterEditionAccount, Metadata, MetadataAccount}, token::{approve, Approve, FreezeAccount, Mint, Token, TokenAccount} };

use crate::{StakeAccount, StakeConfig, UserConfig};

#[derive(Accounts)]
pub struct Stake<'info>{
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
        seeds=[b"metadata",metadata_program.key().as_ref(),mint.key().as_ref()],
        bump,
        seeds::program=metadata_program,
        constraint=metadata.collection.clone().unwrap().key.as_ref()==collection.key().as_ref(),
        constraint=metadata.collection.clone().unwrap().verified==true
    )]
    pub metadata:Account<'info,MetadataAccount>,
    #[account(
        seeds=[b"metadata",metadata_program.key().as_ref(),mint.key().as_ref(),b"edition"],
        bump,
        seeds::program=metadata_program,
    )]
    pub master_edition:Account<'info,MasterEditionAccount>,
    pub collection:Account<'info,Mint>,
    #[account(
        mut,
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
        init,
        payer=user,
        space=8+StakeAccount::INIT_SPACE,
        seeds=[b"stake",user_config.key().as_ref()],
        bump
    )]
    pub stake_account:Account<'info,StakeAccount>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub metadata_program:Program<'info,Metadata>
    

}   


//delegating the authority to the stake_account
//freeze the user nft's ata
//initialize the stake_account
impl<'info>Stake<'info>{
    pub fn stake(&mut self,bumps:StakeBumps)->Result<()>{
        self.initialize_stake_Account(&bumps);
        self.approve();
        self.freeze(&bumps);

        Ok(())
    }
    pub fn freeze(&mut self,bumps:&StakeBumps)->Result<()>{
        let program=self.token_program.to_account_info();
        let account=FreezeDelegatedAccountCpiAccounts{
            delegate:&self.stake_account.to_account_info(),
            edition:&self.master_edition.to_account_info(),
            mint:&self.mint.to_account_info(),
            token_account:&self.user_mint_ata.to_account_info(),
            token_program:&self.token_program.to_account_info()
        };
        let binding = self.user_config.key();
        let seeds=&[
            b"stake",binding.as_ref(),
            &[bumps.stake_account]
        ];
        let signers_seeds=&[&seeds[..]];
        FreezeDelegatedAccountCpi::new(&self.metadata_program.to_account_info(),account).invoke_signed(signers_seeds)?;
        
        Ok(())
    }
    pub fn approve(&mut self)->Result<()>{
        let account=Approve{
            authority:self.user.to_account_info(),
            delegate:self.stake_account.to_account_info(),
            to:self.user_mint_ata.to_account_info()
        };
        let ctx=CpiContext::new(self.token_program.to_account_info(), account);
        approve(ctx, 1)?;
                self.user_config.points=self.user_config.points.saturating_add(self.stake_config.points_per_stake);
        self.user_config.amounts_staked=self.user_config.amounts_staked.saturating_add(1);
        Ok(())
    }
    pub fn initialize_stake_Account(&mut self,bumps:&StakeBumps)->Result<()>{
        self.stake_account.set_inner(StakeAccount { 
            mint: self.mint.key(),
             owner: self.user.key(),
              bump: bumps.stake_account,
               staked_at: Clock::get()?.unix_timestamp
             });
             Ok(())
    }
}