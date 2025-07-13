use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::Config;

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub depositer: Signer<'info>,

    //mints
    #[account(
        mint::token_program=token_program
    )]
    pub mint_x: InterfaceAccount<'info, Mint>,
    #[account(
        mint::token_program=token_program
    )]
    pub mint_y: InterfaceAccount<'info, Mint>,

    //state
    #[account(
        init,
        payer=depositer,
        space=8+Config::INIT_SPACE,
        seeds=[b"config",seed.to_le_bytes().as_ref(),depositer.key().as_ref()],
        bump
    )]
    pub config: Account<'info, Config>,
    #[account(
        init,
        payer=depositer,
        mint::authority=config,
        mint::decimals=6,
        seeds=[b"lp",config.key().as_ref()],
        bump
    )]
    pub mint_lp: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer=depositer,
        associated_token::mint=mint_x,
        associated_token::authority=config,
    )]
    pub vault_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer=depositer,
        associated_token::mint=mint_x,
        associated_token::authority=config,
    )]
    pub vault_y: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, seed: u64,fees:u16, bumps: InitializeBumps) -> Result<()> {
        self.config.set_inner(Config {
            authority: self.depositer.key(),
            mint_x: self.mint_x.key(),
            mint_y: self.mint_y.key(),
            mint_lp: self.mint_lp.key(),
            seed,
            bump: bumps.config,
            lp_bump: bumps.mint_lp,
            fees
        });
        Ok(())
    }
}
