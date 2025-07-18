use anchor_lang::{prelude::*, solana_program::stake::state::Stake};
use anchor_spl::token::{Mint, Token};

use crate::StakeConfig;

#[derive(Accounts)]
pub struct InitializeConfig<'info>{
    #[account(mut)]
    pub admin:Signer<'info>,
    #[account(
        init,
        payer=admin,
        space=8+StakeConfig::INIT_SPACE,
        seeds=[b"config"],
        bump
    )]
    pub stake_config:Account<'info,StakeConfig>,
    #[account(
        init,
        payer=admin,
        seeds=[b"rewards",stake_config.key().as_ref()],
        bump,
        mint::authority=stake_config,
        mint::decimals=6,
        
    )]
    pub rewards_mint:Account<'info,Mint>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>
}

impl<'info>InitializeConfig<'info>{
    pub fn initialize_config(&mut self,points_per_stake:u64,max_amount_staked:u8,fees:u8,freeze_period:u32,bumps:InitializeConfigBumps)->Result<()>{
        self.stake_config.set_inner(StakeConfig {
             points_per_stake,
              max_amount_staked,
               fees,
                rewards_bump:bumps.rewards_mint,
                 stake_config_bump: bumps.stake_config,
                  freeze_period
                 });
                 Ok(())
    }
}