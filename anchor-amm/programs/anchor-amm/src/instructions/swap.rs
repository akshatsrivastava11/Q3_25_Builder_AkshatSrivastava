use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Transfer, Token, TokenAccount, Mint},
};
use constant_product_curve::ConstantProduct;           // or your own math lib

use crate::state::Config;


#[derive(Accounts)]
pub struct Swap<'info>{
    #[account(mut)]
    pub user:Signer<'info>,
    #[account(
              seeds  = [b"config", config.seed.to_le_bytes().as_ref()],
        bump   = config.bump_config,
        has_one = mint_x,
        has_one = mint_y
    )]
    pub config:Account<'info,Config>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,

        #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = config,
    )]
    pub vault_x: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = config,
    )]
    pub vault_y: Account<'info, TokenAccount>,


    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = user,
    )]
    pub user_x: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = user,
    )]
    pub user_y: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

/*----------------------------------------------------- */
/*                   Logic                              */
/*----------------------------------------------------- */

// impl<'info> Swap<'info>{
//     pub 
// }