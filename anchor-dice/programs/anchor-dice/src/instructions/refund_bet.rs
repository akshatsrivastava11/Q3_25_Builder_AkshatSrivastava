use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::Bet;

#[derive(Accounts)]
pub struct RefundBet<'info> {
    #[account(mut)]
    pub gambler: Signer<'info>,
    #[account(
        mut,
        seeds=[b"vault"],
        bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        mut,
        close=gambler,
        seeds=[b"bet",gambler.key().as_ref()],
        bump
    )]
    pub bet: Account<'info, Bet>,
    pub system_program: Program<'info, System>,
}

//transfer amount back + close the bet account
impl<'info>RefundBet<'info> {
    pub fn refund_bet(&mut self,bumps:RefundBetBumps)->Result<()>{
        self.transfer_bet_back(bumps)?;
        Ok(())
    }
    pub fn transfer_bet_back(&mut self,bumps:RefundBetBumps)->Result<()>{
        let seeds:&[&[u8]]=&[
            b"vault",
            &[bumps.vault]
        ];
        let signer_seeds:&[&[&[u8]]]=&[&seeds[..]];
        let accounts=Transfer{
            from:self.vault.to_account_info(),
            to:self.gambler.to_account_info()
        };
        let ctx=CpiContext::new_with_signer(self.system_program.to_account_info(), accounts, signer_seeds);
        transfer(ctx, self.bet.bet_amonut);
        Ok(())
    }
   }
