pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("ALhUBc15mDFXxZyYJPgEXiQJWGRBJP7Pr8vcvdbkFNYW");

#[program]
pub mod anchor_dice {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        ctx.accounts.initialize(amount)?;
        Ok(())
    }
    pub fn place_bet(
        ctx: Context<PlaceBet>,
        bet_amonut: u64,
        roll: u64,
        seed: u8,
    ) -> Result<()> {
        ctx.accounts.place_bet(bet_amonut, roll, seed,  ctx.bumps)?;
        Ok(())
    }
    pub fn refund_bet(ctx:Context<RefundBet>)->Result<()>{
        ctx.accounts.refund_bet(ctx.bumps)?;
        Ok(())
    }
    pub fn resolve_bet(ctx:Context<ResolveBet>,sig:Vec<u8>)->Result<()>{
        ctx.accounts.verift_edd25519_signature(&sig)?;
        ctx.accounts.resolve_bet(&sig, ctx.bumps)?;
        Ok(())
    }

}
