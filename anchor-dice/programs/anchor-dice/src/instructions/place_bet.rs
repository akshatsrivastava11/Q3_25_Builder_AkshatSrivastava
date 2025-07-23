use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::Bet;

#[derive(Accounts)]
#[instruction(seed:u128)]
pub struct PlaceBet<'info>{
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

impl<'info>PlaceBet<'info>{
    pub fn place_bet(&mut self)->Result<()>{
        todo!()
    }
    pub fn create_bet(&mut self,roll:u8,amount:u64,seed:u128,bump:PlaceBetBumps)->Result<()>{
        self.bet.set_inner(Bet {
             player: self.player.key(),
              slot: Clock::get()?.slot,
               roll ,
                amount,
                 seed,
                  bump:bump.bet
                 });
                Ok(())
    }
    pub fn deposit(&mut self,amount:u64)->Result<()>{
        let accounts=Transfer{
            from:self.player.to_account_info(),
            to:self.vault.to_account_info()
        };
        let ctx=CpiContext::new(self.system_program.to_account_info(),accounts);
        transfer(ctx, amount)?;
        Ok(())
    }
}