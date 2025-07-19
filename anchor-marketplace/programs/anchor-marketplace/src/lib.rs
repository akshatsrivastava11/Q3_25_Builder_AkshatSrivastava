pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("DJr74oPyCseMSLvdEE5uZrScMGuXhPVW23TSUaQjuzun");

#[program]
pub mod anchor_marketplace {
    use super::*;
    pub fn initialize(ctx:Context<InitializeMarketplace>,fees:u32,name:String)->Result<()>{
        ctx.accounts.initialize(fees, name, ctx.bumps)
    }
    pub fn list(ctx:Context<List>,price:u64)->Result<()>{
        ctx.accounts.list(price, ctx.bumps)
    }
    pub fn delist(ctx:Context<Delist>)->Result<()>{
        ctx.accounts.delist()
    }
    pub fn purchase(ctx:Context<Purchase>)->Result<()>{
        ctx.accounts.purchase()
    }

}
