use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::Bet;

#[derive(Accounts)]
#[instruction(seed:u128)]
pub struct RefundBet<'info>{
    #[account(mut)]
    pub player:Signer<'info>,
    //CHECK::this is safe
    pub house:UncheckedAccount<'info>,
    #[account(
        init,
        payer=player,
        space=8+Bet::INIT_SPACE,
        seeds=[b"bet",seed.to_be_bytes().as_ref(),vault.key().as_ref()],
        bump
    )]
    pub bet:Account<'info,Bet>,
    #[account(
        mut,
        seeds=[b"vault",house.key().as_ref()],
        bump
    )]
    pub vault:SystemAccount<'info>,
    pub system_program:Program<'info,System>
}

impl<'info> RefundBet<'info>{
    pub fn refund_bet(&mut self,bumps:RefundBetBumps)->Result<()>{
        let accounts=Transfer{
            from:self.vault.to_account_info(),
            to:self.player.to_account_info()
        };
        let binding=self.house.key();
        let seeds=&[
            b"vault",binding.as_ref(),
            &[bumps.vault]
        ];
        let signer_seeds=&[&seeds[..]];
        let ctx=CpiContext::new_with_signer(self.system_program.to_account_info(), accounts, signer_seeds);
        transfer(ctx, self.bet.amount)?;
        Ok(())
    }

}