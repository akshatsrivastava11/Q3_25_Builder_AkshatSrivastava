use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Marketplace{
    pub admin:Pubkey,
    pub fees:u32,
    pub bump:u8,
    pub treasury_bump:u8,
    pub rewards_mint_bump:u8,
    #[max_len(32)]
    pub name:String
}