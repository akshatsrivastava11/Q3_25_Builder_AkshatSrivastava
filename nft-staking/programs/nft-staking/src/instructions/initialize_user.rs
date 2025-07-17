use anchor_lang::{prelude::*, solana_program::clock::SlotIndex};

use crate::UserConfig;

#[derive(Accounts)]
pub struct InitializeUser<'info>{
    #[account(mut)]
    pub  user:Signer<'info>,
    #[account(
        init,
        payer=user,
        space=8+UserConfig::INIT_SPACE,
        seeds=[b"user",user.key().as_ref()],
        bump
    )]
    pub user_config:Account<'info,UserConfig>,
    pub system_program:Program<'info,System>

}

impl<'info>InitializeUser<'info>{
    pub fn initialize_user(&mut self,bumps:InitializeUserBumps)->Result<()>{
        self.user_config.set_inner(UserConfig {
             amount_staked: 0,
              points: 0,
               bump: bumps.user_config 
            });
            Ok(())

    }
}