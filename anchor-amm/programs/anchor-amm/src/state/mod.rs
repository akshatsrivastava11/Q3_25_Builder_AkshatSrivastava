use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config{
    pub mint_x:Pubkey,
    pub mint_y:Pubkey,
    pub authority:Option<Pubkey>,
    pub seed:u64,
    pub bump_config:u8,
    pub bump_lp:u8,
    pub locked:bool,
    pub fees:u16
}

