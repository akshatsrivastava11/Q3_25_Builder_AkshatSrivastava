use anchor_instruction_sysvar::{ed25519, Ed25519InstructionSignature, Ed25519InstructionSignatures};
use anchor_lang::{prelude::*, solana_program::{blake3::hash, ed25519_program, sysvar::instructions::load_instruction_at_checked}, system_program::{transfer, Transfer}};

use crate::Bet;

#[derive(Accounts)]
pub struct ResolveBet<'info>{
    #[account(mut)]
    pub house:Signer<'info>,
    //CHECK:This is safe
    pub player:UncheckedAccount<'info>,
    #[account(
        mut,
        seeds=[b"vault",house.key().as_ref()],
        bump      
    )]
    pub vault:SystemAccount<'info>,
    #[account(
        mut,
        seeds=[b"bet",bet.seed.to_be_bytes().as_ref(),vault.key().as_ref()],
        bump=bet.bump
    )]
    pub bet:Account<'info,Bet>,
    pub instruction_sysvar:AccountInfo<'info>,
    pub system_program:Program<'info,System>

}

impl <'info>ResolveBet<'info> {
    pub fn resolve_bet(&mut self)->Result<()>{
        todo!()
    }
    pub fn verify_ed25519_signature(&mut self,sig:&[u8])->Result<()>{
        //load the first instruction from the transactions
        let ix=load_instruction_at_checked(0, &self.instruction_sysvar.to_account_info())?;
        //making sure the ix is ed25519 ix
        require_keys_eq!(ix.program_id,ed25519_program::ID,DiceError::Ed25519Program);
        //make sure there are no accounts present
        require_eq!(ix.accounts.len(),0,DiceError::Ed25519Accounts);
        let signatures=Ed25519InstructionSignatures::unpack(&ix.data)?.0;
        require_eq!(signatures.len(),1,DiceError::Ed25519DataLength);
        let signature=&signatures[0];
        require!(signature.is_verifiable,DiceError::Ed25519Header);
        require_keys_eq!(signature.public_key.ok_or(DiceError::Ed25519Pubkey)?,self.house.key(),DiceError::Ed25519Pubkey);       
         require!(&signature.signature.ok_or(DiceError::Ed25519Signature)?.eq(sig), DiceError::Ed25519Signature);

        // Ensure messages match
        require!(&signature.message.as_ref().ok_or(DiceError::Ed25519Signature)?.eq(&self.bet.to_slice()), DiceError::Ed25519Signature);

        Ok(())
    }
    pub fn resolve(&mut self,sig:&[u8],bumps:ResolveBetBumps)->Result<()>{
        let hash=hash(sig).to_bytes();
        let mut hash16=[0;16];
        hash16.copy_from_slice(&hash[0..16]);
        let lower=u128::from_le_bytes(hash16);
        hash16.copy_from_slice(&hash[16..32]);
        let upper=u128::from_le_bytes(hash16);
        let roll=lower.wrapping_add(upper).wrapping_rem(100) as u8 +1;
        if self.bet.roll>=roll{
 let payout = (self.bet.amount as u128)
            .checked_mul(10000 - 1000 as u128).ok_or(DiceError::Overflow)?
            .checked_div(self.bet.roll as u128 - 1).ok_or(DiceError::Overflow)?
            .checked_div(100).ok_or(DiceError::Overflow)? as u64;
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
                        let ctx = CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                accounts,
                signer_seeds
            );
            transfer(ctx, payout)?;
                }
                Ok(())
    }

}

#[error_code]
pub enum DiceError {
    #[msg("Bump error")]
    BumpError,
    #[msg("Overflow")]
    Overflow,
    #[msg("Minimum bet is 0.01 Sol")]
    MinimumBet,
    #[msg("Maximum bet exceeded")]
    MaximumBet,
    #[msg("Minimum roll is 2")]
    MinimumRoll,
    #[msg("Maximum roll is 96")]
    MaximumRoll,
    #[msg("Timeout not yet reached")]
    TimeoutNotReached,
    #[msg("Ed25519 Header Error")]
    Ed25519Header,
    #[msg("Ed25519 Pubkey Error")]
    Ed25519Pubkey,
    #[msg("Ed25519 Message Error")]
    Ed25519Message,
    #[msg("Ed25519 Signature Error")]
    Ed25519Signature,
    #[msg("Ed25119 Program Error")]
    Ed25519Program,
    #[msg("Ed25119 Accounts Error")]
    Ed25519Accounts,
    #[msg("Ed25119 Data Length Error")]
    Ed25519DataLength
}