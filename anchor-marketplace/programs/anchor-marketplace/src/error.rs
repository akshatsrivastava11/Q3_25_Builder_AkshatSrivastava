use anchor_lang::prelude::*;

#[error_code]
pub enum MarketPlaceError {
    #[msg("Name is either too long/short")]
    NameError,
}
