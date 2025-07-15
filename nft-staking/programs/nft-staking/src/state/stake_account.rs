use anchor_lang::prelude::*;

pub struct StakeAccount{
    pub mint:Pubkey,
    pub owner:Pubkey,
    pub staked_at:i64,
    pub bump:u8
}