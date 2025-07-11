pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("6DMqb98TyrzSbXjsmJNtHyrJGm3j2dwqZAv8NfwDbjvj");

#[program]
pub mod anchor_amm {

    use super::*;

    pub fn initialize(ctx:Context<Initialize>,seed:u64,fees:u16)->Result<()>{
        ctx.accounts.initialize(seed,fees,ctx.bumps)
    }
    pub fn deposit(ctx:Context<Deposit>,amount:u64,max_x:u64,max_y:u64)->Result<()>{
        ctx.accounts.deposit(amount, max_x, max_y)
    }
    pub fn withdraw(ctx:Context<Withdraw>,amountLp:u64,max_x:u64,max_y:u64)->Result<()>{
        ctx.accounts.withdraw(amountLp, max_x, max_y)
    }
    pub fn swap(ctx:Context<Swap>,amount_in:u64,min_out:u64,is_x_in:bool)->Result<()>{
        ctx.accounts.swap(amount_in, min_out, is_x_in)
    }
 
}
