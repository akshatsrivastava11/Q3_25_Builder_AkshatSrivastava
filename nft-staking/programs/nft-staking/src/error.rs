use anchor_lang::prelude::*;

#[error_code]
pub enum StakeError {
    #[msg("Time Has not elapsed for unstaking")]
    TimeElapsedError,
    #[msg("No Nft staked")]
    NoNFTStakedError
    
}

