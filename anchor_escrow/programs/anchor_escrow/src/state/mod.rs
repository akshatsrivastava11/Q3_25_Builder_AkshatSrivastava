use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow{
    pub maker:Pubkey,
    pub mint_a:Pubkey,
    pub mint_b:Pubkey,
    pub amount:u64,
    pub bump:u8,
}
