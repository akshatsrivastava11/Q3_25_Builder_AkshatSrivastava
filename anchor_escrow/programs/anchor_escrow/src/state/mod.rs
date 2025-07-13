use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow{
    pub maker:Pubkey,
    pub mint_x:Pubkey,
    pub mint_y:Pubkey,
    pub amonunt:u64,
    pub escrow_bump:u8,
    pub seed:u64,
}