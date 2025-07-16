use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeAccount{
    pub mint:Pubkey,
    pub owner:Pubkey,
    pub staked_at:u64,
    pub bump:u8
}