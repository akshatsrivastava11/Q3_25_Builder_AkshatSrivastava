use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Stake<'info>{
    #[account(mut)]
    pub staker:Signer<'info>,

}