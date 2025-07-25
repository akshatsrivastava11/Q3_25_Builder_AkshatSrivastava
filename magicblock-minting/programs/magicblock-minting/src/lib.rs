use anchor_lang::prelude::*;

declare_id!("uZb3Zy1cyxGZ2BqL89odWfxiNHrvHGeCTPGsgYPS6oD");
pub mod instruction;
pub use instruction::*;

#[program]
pub mod magicblock_minting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
