use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::Bet;

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub gambler: Signer<'info>,
    #[account(
        mut,
        seeds=[b"vault"],
        bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        init,
        payer=gambler,
        space=8+Bet::INIT_SPACE,
        seeds=[b"bet",gambler.key().as_ref()],
        bump
    )]
    pub bet: Account<'info, Bet>,
    pub system_program: Program<'info, System>,
}
impl<'info> PlaceBet<'info> {
    pub fn place_bet(
        &mut self,
        bet_amonut: u64,
        roll: u64,
        seed: u8,
        bumps: PlaceBetBumps,
    ) -> Result<()> {
        self.make_bet(bet_amonut, roll, seed, bumps);
        self.transfer_bet(bet_amonut);
        Ok(())
    }
    pub fn make_bet(
        &mut self,
        bet_amonut: u64,
        roll: u64,
        seed: u8,
        bumps: PlaceBetBumps,
    ) -> Result<()> {
        self.bet.set_inner(Bet {
            bet_amonut,
            roll,
            gambler: self.gambler.key(),
            seed,
            bump: bumps.bet,
            slot:Clock::get()?.slot,
        });

        Ok(())
    }
    pub fn transfer_bet(&mut self, bet_amonut: u64) -> Result<()> {
        let accounts = Transfer {
            from: self.gambler.to_account_info(),
            to: self.vault.to_account_info(),
        };
        let program = self.system_program.to_account_info();
        let ctx = CpiContext::new(program, accounts);
        transfer(ctx, bet_amonut);
        Ok(())
    }
}
