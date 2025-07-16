use anchor_lang::prelude::*;



#[account]
#[derive(InitSpace)]
pub struct UserConfig{
       pub token_number_staked:u8,
       pub points:u8,
       pub bumps:u8,
}