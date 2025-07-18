use anchor_lang::{prelude::*};
use anchor_spl::{associated_token::AssociatedToken, metadata::{mpl_token_metadata::instructions::{FreezeDelegatedAccount, FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts}, MasterEditionAccount, Metadata, MetadataAccount}, token::{approve, mint_to, Approve, FreezeAccount, Mint, MintTo, Token, TokenAccount}};

use crate::{stake_config, StakeAccount, StakeConfig, UserConfig};

#[derive(Accounts)]
pub struct Claim<'info>{
    #[account(mut)]
    pub user:Signer<'info>,
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
        seeds=[b"rewards",stake_config.key().as_ref()],
        bump,
        mint::authority=stake_config,
        mint::decimals=6,
    )]
    pub rewards:Account<'info,Mint>,
    #[account(
        init_if_needed,
        payer=user,
        associated_token::authority=user,
        associated_token::mint=rewards
    )]
    pub rewards_ata_user:Account<'info,TokenAccount>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub associated_token_program:Program<'info,AssociatedToken>
}
impl<'info>Claim<'info>{
    pub fn claim(&mut self,bumps:ClaimBumps)->Result<()>{
        let claimtoken=self.user_config.points*10_u64.pow(self.rewards.decimals as u32);
        let program=self.token_program.to_account_info();
        let seeds:&[&[u8]]=&[
            b"config",
            &[self.stake_config.stake_config_bump]
        ];
        let signer_seeds=&[&seeds[..]];
        let acccounts=MintTo{
            authority:self.stake_config.to_account_info(),
            mint:self.rewards.to_account_info(),
            to:self.rewards_ata_user.to_account_info()
        };
        let ctx=CpiContext::new_with_signer(program, acccounts, signer_seeds);
        mint_to(ctx, claimtoken);
        self.user_config.points=0;
        Ok(())
    }
}