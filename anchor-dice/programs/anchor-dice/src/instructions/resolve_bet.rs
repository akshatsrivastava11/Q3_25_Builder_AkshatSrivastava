use anchor_lang::{prelude::*, solana_program::{blake3::hash, ed25519_program, sysvar::instructions::load_instruction_at_checked}, system_program::{transfer, Transfer}};
use anchor_instruction_sysvar::{Ed25519InstructionSignature, Ed25519InstructionSignatures};
use crate::Bet;

#[derive(Accounts)]
pub  struct ResolveBet<'info>{
    #[account(mut)]
    pub house:Signer<'info>,

    #[account(mut)]
    ///CHECK:This is safe
    pub gambler:UncheckedAccount<'info>,
    #[account(
        seeds=[b"bet",gambler.key().as_ref()],
        bump        
    )]
    pub bet:Account<'info,Bet>,
    #[account(
        mut,
        seeds=[b"vault"],
        bump
    )]
    pub vault:SystemAccount<'info>,
    pub system_program:Program<'info,System>,
    ///CHECK:This is safe
    pub instruction_sysvar:AccountInfo<'info>
}

impl<'info>ResolveBet<'info>{
    //checks for the instruction sysvar
    pub fn verift_edd25519_signature(&mut self,sig:&[u8])->Result<()>{
        let ix=load_instruction_at_checked(0, &self.instruction_sysvar.to_account_info())?;
        //checks if the instruction_sysvar is key
        require_keys_eq!(ix.program_id,ed25519_program::ID,DiceError::ED25519KEYERROR);
        //checks for no account is there
        require_eq!(ix.accounts.len(),0,DiceError::ED25519ACCOUNTLENERROR);
        //checks for only one signer
        let signatures=Ed25519InstructionSignatures::unpack(&ix.data)?.0;

        require_eq!(signatures.len(),1,DiceError::ED25519SIGNERLENERROR);
        let signature=&signatures[0];
        require!(signature.is_verifiable,DiceError::ED25519SIGNATUREUNVERIFIED);
        require_keys_eq!(signature.public_key.ok_or(DiceError::Ed25519Pubkey)?,self.house.key(),DiceError::ED25519SIGNATUREMISMATCH);  
        require!(&signature.signature.ok_or(DiceError::Ed25519SIGNATURE)?.eq(sig), DiceError::Ed25519SIGNATURE);
        require!(&signature.message.as_ref().ok_or(DiceError::Ed25519SIGNATURE)?.eq(&self.bet.to_slice()),DiceError::Ed25519SIGNATURE);
        Ok(())
    }
    //generating random no. and transfering token
    pub fn resolve_bet(&mut self,sig:&[u8],bumps:ResolveBetBumps)->Result<()>{
        let hash=hash(sig).to_bytes();
        let mut firstHalf=[0;16];
        firstHalf.copy_from_slice(&hash[0..16]);
        let mut second_half=[0;16];
        second_half.copy_from_slice(&hash[16..32]);
        let upper=u128::from_le_bytes(firstHalf);
        let lower=u128::from_le_bytes(second_half);
        let roll=upper.wrapping_add(lower).wrapping_rem(100) as u8 +1;
        //if program generated roll is less than what the user gueses,user won
        if roll<self.bet.roll as u8{
            let payout=self.bet.bet_amonut.checked_mul(10000-1000).ok_or(DiceError::OVERFLOW)?.checked_div(self.bet.roll).ok_or(DiceError::OVERFLOW)?.checked_div(100).ok_or(DiceError::OVERFLOW)?;
            let accounts=Transfer{
                from:self.vault.to_account_info(),
                to:self.gambler.to_account_info()
            };
        let seeds:&[&[u8]]=&[
            b"vault",
            &[bumps.vault]
        ];
        let signer_seeds:&[&[&[u8]]]=&[&seeds[..]];
        let ctx=CpiContext::new_with_signer(self.system_program.to_account_info(), accounts, signer_seeds);
        transfer(ctx, payout);
        }
        Ok(())
    }

}

#[error_code]
pub enum DiceError{
    #[msg("Not a ED25519 Key")]
    ED25519KEYERROR,
    #[msg("Contains Account ")]
    ED25519ACCOUNTLENERROR,
    #[msg("More than signer ")]
    ED25519SIGNERLENERROR,
    #[msg("Signature unverified")]
    ED25519SIGNATUREUNVERIFIED,
    #[msg("Signature PublicKey Not matched to house")]
    ED25519SIGNATUREMISMATCH,
    #[msg("NO public key")]
    Ed25519Pubkey,
    #[msg("Signature message not as expected")]
    Ed25519SIGNATURE,
    #[msg("calculation overflowed")]
    OVERFLOW,
    
}