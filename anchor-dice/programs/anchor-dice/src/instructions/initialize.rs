use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(mut)]
    pub house:Signer<'info>,
    #[account(
        mut,
        seeds=[b"vault"],
        bump
    )]
    pub vault:SystemAccount<'info>,
    pub system_program:Program<'info,System>
}

impl<'info>Initialize<'info>{
    pub fn initialize(&mut self,amount:u64)->Result<()>{
        let program=self.system_program.to_account_info();
        let accounts=Transfer{
            from:self.house.to_account_info(),
            to:self.vault.to_account_info()
        };
        let ctx=CpiContext::new(program, accounts);
        msg!("Transferring to the vault from the house {}",amount);
        transfer(ctx, amount)?;
        Ok(())
    }
}