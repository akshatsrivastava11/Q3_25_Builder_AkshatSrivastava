use anchor_lang::{prelude::*, solana_program::stake::state::Stake};

use crate::StakeConfig;

#[derive(Accounts)]
pub struct InitilizeConfig<'info>{
    #[account(mut)]
    pub signer:Signer<'info>,
    //config
    #[account(
        init,
        payer=signer,
        space=8+StakeConfig::INIT_SPACE,
        seeds=[b"config"],
        bump
    )]
    pub config:Account<'info,StakeConfig>,
    #[account(
        init,
        payer=signer,
        seeds=[b"rewards",config.key().as_ref()],
        bump,
        mint::decimals=6,
        mint::authority=config
    )]
    pub reward_mint:Account<'info,Mint>,
    pub system_program:Program<'info,System>
}

impl<'info> InitilizeConfig<'info> {
    pub fn initialize_config(&mut self,points_per_stake:u8,max_stake:u8,freeze_period:u32,bumps:InitilizeConfigBumps)->Result<()>{
        self.config.set_inner(StakeConfig {
             points_per_stake,
              max_stake, 
              freeze_period,
               bump: bumps.config,
                reward_bump: bumps.reward_mint
             });
        Ok(())
    }
}