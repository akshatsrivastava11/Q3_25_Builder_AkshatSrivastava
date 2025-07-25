use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{mint_to, Mint, MintTo, Token, TokenAccount}};


#[derive(Accounts)]
pub struct MintToken<'info>{
    #[account(mut)]
    pub payer:Signer<'info>,
    #[account(
        mint::decimals=6,
        mint::authority=mint.key(),
        mint::freeze_authority=mint.key()
    )]
    pub mint:Account<'info,Mint>,
    #[account(
        init_if_needed,
        payer=payer,
        associated_token::mint=mint,
        associated_token::authority=payer
    )]
    pub  token_account:Account<'info,TokenAccount>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub associated_token_program:Program<'info,AssociatedToken>
}
impl<'info>MintToken<'info>{
    pub fn mint_token(&mut self,amount:u64)->Result<()>{
        let accounts=MintTo{
            authority:self.mint.to_account_info(),
            mint:self.mint.to_account_info(),
            to:self.token_account.to_account_info()
        };
        let seeds:&[&[u8]]=&[b"mint"];
        let signer_seeds:&[&[&[u8]]]=&[&seeds[..]];
        let ctx=CpiContext::new_with_signer(self.token_program.to_account_info(), accounts, signer_seeds);
        mint_to(ctx, amount*10u64.pow(self.mint.decimals as u32));
        Ok(())
    }
}