use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserConfig{
       pub points:u64,
       pub amounts_staked:u64,
       pub bump:u8
}