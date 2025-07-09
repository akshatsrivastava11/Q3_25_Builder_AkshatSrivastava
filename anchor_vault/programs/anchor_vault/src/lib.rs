use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
declare_id!("EE8yan9EEzN7zVfrsNnqdwsr5ZVAVeiJC52s6muQvbWN");

#[program]
pub mod anchor_vault {
     use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initalize(&ctx.bumps)
    }

    // insted of deposit and withdraw we use payment as common term because both have same accounts struct and we can impl multiple fun
    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()>{
        ctx.accounts.deposit(amount)
    }

    // withdraw
    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()>{
        ctx.accounts.withdraw(amount)
    }

    // close acc and withdraw all lamports in user wallet
    pub fn close(ctx: Context<Close>) -> Result<()>{
        ctx.accounts.close()
    }
}

// write the accounts that need for initalize so for vault we need a vault sys acc or vault_state to store bump onchain, sysyem program to handle cration of acc and other transaction
// account struct for initalize instruction = matlab kon kon se acc use hoge
#[derive(Accounts)]
pub struct Initialize<'info>{
    // signer
    #[account(mut)]
    pub user: Signer<'info>,
    // valet_state account
    #[account(
        init,
        payer= user,
        seeds=[b"state", user.key().as_ref()],
        bump,
        space=VaultState::INIT_SPACE
    )]
    pub vault_state: Account<'info, VaultState>,

    // vault
    #[account(
        seeds= [b"vault", user.key().as_ref()],
        bump
    )]
    pub vault : SystemAccount<'info>,

    // system program
    pub system_program : Program<'info, System>
}

// how the initalize work -> how client or what client call -> contains fn 
impl<'info> Initialize<'info> {
    pub fn initalize(&mut self, bumps: &InitializeBumps) -> Result<()>{
        self.vault_state.bump_state = bumps.vault_state; 
        self.vault_state.bump_vault = bumps.vault; 

        Ok(())
    }
}

// depost acc struct -> contains acc that use in deposit -> user (sign), sytem pro(transfer fund), vault_state(), vault
#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds= [b"state", user.key().as_ref()],
        bump = vault_state.bump_state
    )]
    pub vault_state : Account<'info, VaultState>,

    #[account(
        seeds= [b"vault", user.key().as_ref()],
        bump = vault_state.bump_vault 
    )]
    pub vault: SystemAccount<'info>,

    pub system_program : Program<'info, System>
}

// impl the deposit fun on it
// deposit krne ke liye Transfer struct (from, to) and tranfer fun ki jrurat hogi
impl<'info> Payment<'info>{
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        
        let cpi_program = self.system_program.to_account_info();
        
        let cpi_accounts = Transfer{
            from : self.user.to_account_info(),
            to : self.vault.to_account_info()
        };
   
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_context, amount)
    } 

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        
        // which account are we using or will we used here
        let cpi_accounts = Transfer{
            from : self.vault.to_account_info(),
            to : self.user.to_account_info()
        };

        let binding = self.user.key();
        let seeds = &[
            b"vault", 
            binding.as_ref(),
            &[self.vault_state.bump_vault] 
        ];

        // new _signer we need seeds for signing pda
        let signer_seeds = &[&seeds[..]];
        // new_signer when we sign with pda
        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_context, amount)
    }
}

// close account
#[derive(Accounts)]
pub struct Close<'info>{
    #[account(mut)]
    pub user: Signer<'info>,

    // because we closeing means mutating mean state change then we inclue mut in contstraints and also add close = user for closing it
    #[account(
        mut,
        seeds= [b"state", user.key().as_ref()],
        bump = vault_state.bump_state,
        close = user
    )]
    pub vault_state: Account<'info, VaultState>,

    // we do not need to close sys acc because it automatically close
    #[account(
        seeds = [b"vault", user.key().as_ref()],
        bump = vault_state.bump_vault
    )]
    pub vault: SystemAccount<'info>,

    pub system_program : Program<'info ,System>
}

impl<'info> Close<'info> {
    pub fn close(&self) -> Result<()> {
        // program that need for cpi 
        let cpi_program = self.system_program.to_account_info();

        // accounts that needed for cpi
        let cpi_accounts = Transfer{
            from: self.vault.to_account_info(),
            to: self.user.to_account_info()
        };

        let binding = self.user.key();
        let seeds = [
            b"vault", 
            binding.as_ref(),
            &[self.vault_state.bump_vault]
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        // get the remaining lamports and close the account
        // trnsfer all money 
        transfer(cpi_context, self.vault.lamports())
    } 
}

// VaultState ka type 
#[account]
pub struct VaultState {
    pub bump_state: u8,
    pub bump_vault: u8
}

impl Space for VaultState {
    const INIT_SPACE: usize = 8 + 1 + 1; // 8 for discrimator, 1 for bump_state (u8=8bit=1byte) or 1 for bump_vault
}