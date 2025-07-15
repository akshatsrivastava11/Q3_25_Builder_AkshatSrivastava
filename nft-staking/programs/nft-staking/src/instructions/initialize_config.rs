use anchor_lang::prelude::*;

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
        mint::decimals=6,
        mint::authority=config
    )]
    pub reward_mint:Account<'info,Mint>,
    pub system_program:Program<'info,System>
}

impl<'info> InitilizeConfig<'info> {
    pub fn initialize_config(&mut self)->Result<()>{
        
        todo!()
    }
}