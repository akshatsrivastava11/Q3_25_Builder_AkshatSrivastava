use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config{
    pub authority:Pubkey,
    pub mint_x:Pubkey,
    pub mint_y:Pubkey,
    pub mint_lp:Pubkey,
    pub seed:u64,
    pub bump:u8,
    pub lp_bump:u8,
    pub fees:u16
}
