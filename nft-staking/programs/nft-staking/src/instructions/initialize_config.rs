use anchor_lang::prelude::*;

use crate::StakeConfig;

#[derive(Accounts)]
pub struct InitializeConfig<'info>{
    #[account(mut)]
    pub user:Signer<'info>,
    #[account(
        init,
        payer=user,
        space=8+StakeConfig::INIT_SPACE,
        seeds=[b"config".as_ref()],
        bump
    )]
    pub config:Account<'info,StakeConfig>,
    #[account(
        init,
        payer=user,
        mint::decimals=6,
        mint::authority=config,
        seeds=[b"reward".as_ref(),config.key().as_ref()],
        bump
    )]
    pub reward:Account<'info,Mint>,
    pub system_program:Program<'info,System>

}

impl<'info>InitializeConfig<'info>{
    pub fn initialize_config(&mut self,points_per_stake:u8,max_stake:u8,freeze_period:u32,bumps:InitializeConfigBumps)->Result<()>{
        self.config.set_inner(StakeConfig { 
            points_per_stake,
             max_stake, 
             freeze_period,
              config_bump:bumps.config,
               rewards_bump: bumps.reward
             });
             Ok(())
    }
}