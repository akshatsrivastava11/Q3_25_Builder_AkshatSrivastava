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

   pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        fees: u16,
        authority: Option<Pubkey>,
    ) -> Result<()> {
        ctx.accounts.initialize(seed, fees, authority, ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(
        ctx: Context<Deposit>,
        amount: u64,
        max_x: u64,
        max_y: u64,
    ) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y)
    }
         pub fn withdraw(
        ctx: Context<Withdraw>,
        lp_amount: u64,
        min_x:  u64,
        min_y: u64,
    ) -> Result<()> {
        ctx.accounts.withdraw_token(lp_amount, min_x,min_y)
    }
}
