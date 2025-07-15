use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserConfig{
       pub points:u32,
       pub max_stake:u8,
       pub bump:u8
}