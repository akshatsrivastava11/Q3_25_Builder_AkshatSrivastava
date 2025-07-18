use anchor_lang::{prelude::*, solana_program::stake::config::Config};

use crate::{StakeConfig, UserConfig};
#[derive(Accounts)]
pub struct InitializeUser<'info>{
    #[account(mut)]
    pub user:Signer<'info>,
    #[account(
        seeds=[b"config"],
        bump=stake_config.stake_config_bump
    )]
    pub stake_config:Account<'info,StakeConfig>,
    #[account(
        init,
        payer=user,
        space=8+UserConfig::INIT_SPACE,
        seeds=[b"user",stake_config.key().as_ref(),user.key().as_ref()],
        bump,
    )]
    pub user_config:Account<'info,UserConfig>,
    pub system_program:Program<'info,System>
}

impl <'info>InitializeUser<'info> {
    pub fn initialize_user(&mut self,bumps:InitializeUserBumps)->Result<()>{
        self.user_config.set_inner(UserConfig {
             points: 0, 
             amounts_staked: 0,
              bump: bumps.user_config
             });
        
        Ok(())
    }
}