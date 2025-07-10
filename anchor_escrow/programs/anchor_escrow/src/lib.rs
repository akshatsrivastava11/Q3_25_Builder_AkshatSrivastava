pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("GZm78V2rHQuZfCY1iNhb3jwNfUuWQuf1LdivDdeCkNVP");

#[program]
pub mod anchor_escrow {
    use super::*;
    pub fn make(ctx:Context<Make>,amountMake:u64,amountTake:u64)->Result<()>{
        ctx.accounts.initialize(amountTake, ctx.bumps);
        ctx.accounts.make_offer(amountMake)
    }
    pub fn refund(ctx:Context<Refund>)->Result<()>{
        ctx.accounts.refund_and_close()
    }
    pub fn take(ctx:Context<Take>)->Result<()>{
        ctx.accounts.take()
    }
}
