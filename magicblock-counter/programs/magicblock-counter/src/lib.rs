use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::anchor::{commit, delegate};

declare_id!("75efnsDGzobNSdKXLkHTJDb4TVdN9TRJz6pFS4CPQ4K3");

#[program]
pub mod magicblock_counter {
    use ephemeral_rollups_sdk::{cpi::DelegateConfig, ephem::{commit_accounts, commit_and_undelegate_accounts}};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter=&mut ctx.accounts.counter;
        counter.counter=0;
        msg!("Counter is {}",counter.counter);
        Ok(())
    }
    pub fn incrementCounter(ctx:Context<IncrementCounter>)->Result<()>{
        ctx.accounts.counter.counter+=1;
        msg!("Counter is {}",ctx.accounts.counter.counter);

        Ok(())
    }
    pub fn delegate(ctx:Context<DelegateInput>)->Result<()>{
        ctx.accounts.delegate_pda(&ctx.accounts.payer, &[b"Counter"], DelegateConfig::default())?;
        Ok(())
    }
    pub fn commit(ctx:Context<IncrementAndCommit>)->Result<()>{
        commit_accounts(&ctx.accounts.signer, 
            vec![&ctx.accounts.counter.to_account_info()]
            , &ctx.accounts.magic_context
            , &ctx.accounts.magic_program
        )?;
        Ok(())
    }
    pub fn undelegate(ctx:Context<IncrementAndCommit>)->Result<()>{
        commit_and_undelegate_accounts(&ctx.accounts.signer, 
            vec![&ctx.accounts.counter.to_account_info()]
            , &ctx.accounts.magic_context
            , &ctx.accounts.magic_program
        )?;
        Ok(())
    }
    pub fn increment_and_commit(ctx:Context<IncrementAndCommit>)->Result<()>{
        let counter=&mut ctx.accounts.counter;
        counter.counter+=1;
        msg!("The counter is {}",counter.counter);
              commit_accounts(&ctx.accounts.signer, 
            vec![&ctx.accounts.counter.to_account_info()]
            , &ctx.accounts.magic_context
            , &ctx.accounts.magic_program
        )?;
        Ok(())
    }
    pub fn increment_and_undelegate(ctx:Context<IncrementAndCommit>)->Result<()>{
        let counter=&mut ctx.accounts.counter;
        counter.counter+=1;
        msg!("The counter is {}",counter.counter);
               commit_and_undelegate_accounts(&ctx.accounts.signer, 
            vec![&ctx.accounts.counter.to_account_info()]
            , &ctx.accounts.magic_context
            , &ctx.accounts.magic_program
        )?;
        Ok(())
    }
}



#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(mut)]
    pub signer:Signer<'info>,
    #[account(
        init,
        payer=signer,
        space=8+8,
        seeds=[b"Counter"],
        bump
    )]
    pub counter:Account<'info,Counter>,
    pub system_program:Program<'info,System>
}


#[derive(Accounts)]
pub struct IncrementCounter<'info>{
    #[account(
        mut,
        seeds=[b"Counter"],
        bump

    )]
    pub counter:Account<'info,Counter>
}

#[delegate]
#[derive(Accounts)]
pub struct DelegateInput<'info>{
    pub payer:Signer<'info>,
    #[account(mut,del)]
    pub pda:AccountInfo<'info>
}


#[commit]
#[derive(Accounts)]
pub struct IncrementAndCommit<'info>{
    #[account(mut)]
    pub signer:Signer<'info>,
    #[account(
        mut,
        seeds=[b"Counter"],
        bump
    )]
    pub counter:Account<'info,Counter>

}

#[account]
pub struct Counter{
    pub counter:u64
}