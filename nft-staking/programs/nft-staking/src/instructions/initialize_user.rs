use anchor_lang::prelude::*;

use crate::UserConfig;



#[derive(Accounts)]
pub struct InitilizeUser<'info>{
    #[account(mut)]
    pub user:Signer<'info>,
    #[account(
        init,
        payer=user,
        space=8+UserConfig::INIT_SPACE,
        seeds=[b"user",user.key().as_ref()],
        bump
    )]
    pub user_account:Account<'info,UserConfig>,
    pub system_program:Program<'info,System>
}


impl<'info>InitilizeUser<'info>{
    pub fn initialize_user(&mut self,bumps:InitilizeUserBumps)->Result<()>{
        self.user_account.set_inner(UserConfig { 
            points: 0,
             max_stake: 0,
              bump: bumps.user_account
             });
             Ok(())
    }
}