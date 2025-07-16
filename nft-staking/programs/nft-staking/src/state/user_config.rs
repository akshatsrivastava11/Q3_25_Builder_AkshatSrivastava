use anchor_lang::prelude::*;



#[account]
#[derive(InitSpace)]
pub struct UserConfig{
       pub amount_staked:u8,
       pub points:u32,
       pub bump:u8,
}