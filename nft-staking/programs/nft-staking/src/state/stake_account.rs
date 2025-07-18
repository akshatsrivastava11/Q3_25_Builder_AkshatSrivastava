use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeAccount{
    pub mint:Pubkey,
    pub owner:Pubkey,
    pub bump:u8,
    pub staked_at:i64
}