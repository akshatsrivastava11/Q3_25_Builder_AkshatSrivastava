use anchor_lang::prelude::*;
use anchor_spl::{token::Token, token_interface::{Mint, TokenInterface}};


use crate::error::MarketPlaceError;
use crate::Marketplace;
#[derive(Accounts)]
#[instruction(name:String)]
pub struct InitializeMarketplace<'info>{
    #[account(mut)]
    pub admin:Signer<'info>,
    #[account(
        seeds=[b"treasury",marketplace.key().as_ref()],
        bump
    )]
    pub treasury:SystemAccount<'info>,
    #[account(
        init,
        payer=admin,
        space=8+Marketplace::INIT_SPACE,
        seeds=[b"marketplace",name.as_bytes()],
        bump
    )]
    pub marketplace:Account<'info,Marketplace>,
    #[account(
        init,
        payer=admin,
        seeds=[b"rewards",marketplace.key().as_ref()],
        bump,
        mint::authority=marketplace,
        mint::decimals=6
    )]
    pub rewards:InterfaceAccount<'info,Mint>,
    pub system_program:Program<'info,System>,
    pub token_program:Interface<'info,TokenInterface>
}

impl<'info>InitializeMarketplace<'info>{
    pub fn initialize(&mut self,fees:u32,name:String,bumps:InitializeMarketplaceBumps)->Result<()>{
        require!(!name.is_empty() && name.len()<=32,MarketPlaceError::NameError);
        self.marketplace.set_inner(Marketplace { 
            admin:self.admin.key(),
             fees,
              bump: bumps.marketplace,
               treasury_bump: bumps.treasury,
                rewards_mint_bump: bumps.rewards,
                 name
                 });
                 Ok(())

    }
}