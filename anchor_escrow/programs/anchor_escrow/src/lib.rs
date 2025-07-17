pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("3jjViV2cphNabK7gbWZcSic4R2LihFJ8G8qH535Zc28U");

#[program]
pub mod anchor_escrow {
    use super::*;
    pub fn make(ctx:Context<Make>,seed:u64,amount_to_be_recieved:u64,amount_to_be_deposited:u64)->Result<()>{
        msg!("SEed is {}",seed);
        
        ctx.accounts.initilize(amount_to_be_recieved,seed,ctx.bumps);
        ctx.accounts.make(amount_to_be_deposited)
    }
    pub fn refund(ctx:Context<Refund>)->Result<()>{
        ctx.accounts.transfer();
        ctx.accounts.close()
    }
    pub fn take(ctx:Context<Take>)->Result<()>{
        ctx.accounts.transfer_to_maker_mint_y();
        ctx.accounts.transfer_from_vault_to_taker_mint_x()
    }
}
