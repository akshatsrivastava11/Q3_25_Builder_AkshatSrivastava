pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("CbTPpZ1dECkSyELUZTKZjXynp6Zecwf6wCz8Yu3xvt7g");

#[program]
pub mod nft_staking {
    use super::*;
    pub fn initialize(ctx:Context<InitializeConfig>,points_per_stake:u64,max_amount_staked:u8,fees:u8,freeze_period:u32)->Result<()>{
        ctx.accounts.initialize_config(points_per_stake, max_amount_staked, fees, freeze_period,ctx.bumps)
    }
    pub fn initilialize_user(ctx:Context<InitializeUser>)->Result<()>{
        ctx.accounts.initialize_user(ctx.bumps)
    }
    pub fn stake(ctx:Context<Stake>)->Result<()>{
        ctx.accounts.stake(ctx.bumps)
    }
    pub fn unstake(ctx:Context<Unstake>)->Result<()>{
        ctx.accounts.unstake(ctx.bumps)
    }
    pub fn claim(ctx:Context<Claim>)->Result<()>{
        ctx.accounts.claim(ctx.bumps)
    }
}
