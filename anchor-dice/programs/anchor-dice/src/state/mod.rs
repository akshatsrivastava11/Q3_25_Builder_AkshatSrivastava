use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Bet{
    pub gambler:Pubkey,
    pub bet_amonut:u64,
    pub roll:u64,
    pub seed:u8,
    pub bump:u8,
    pub slot:u64
}

impl Bet{
    pub fn to_slice(&self)->Vec<u8>{
        let mut s=self.gambler.to_bytes().to_vec();
        s.extend_from_slice(&self.bet_amonut.to_le_bytes());
        s.extend_from_slice(&self.roll.to_le_bytes());
        s.extend_from_slice(&self.seed.to_le_bytes());
        s.extend_from_slice(&self.bump.to_le_bytes());
        s.extend_from_slice(&self.slot.to_le_bytes());
        return s;
    }
}