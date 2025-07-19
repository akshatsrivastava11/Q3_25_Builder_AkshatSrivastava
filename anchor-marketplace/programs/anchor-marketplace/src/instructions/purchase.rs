use anchor_lang::{prelude::*, system_program::{Transfer,transfer}};
use anchor_spl::{associated_token::AssociatedToken, token::{ close_account, mint_to, transfer_checked, CloseAccount, MintTo, Token, TransferChecked}, token_2022::spl_token_2022::extension::metadata_pointer::processor, token_interface::{Mint, TokenInterface,TokenAccount}};

use crate::{marketplace, Listing, Marketplace};

#[derive(Accounts)]
pub struct Purchase<'info>{
    #[account(mut)]
    pub taker:Signer<'info>,
    pub maker:SystemAccount<'info>,
    pub mint:InterfaceAccount<'info,Mint>,
    #[account(
        mut,
        associated_token::authority=maker,
        associated_token::mint=mint
    )]
    pub maker_ata:InterfaceAccount<'info,TokenAccount>,
    #[account(
        mut,
        associated_token::authority=maker,
        associated_token::mint=mint
    )]
    pub taker_ata:InterfaceAccount<'info,TokenAccount>,
    #[account(
        mut,
        associated_token::authority=listing,
        associated_token::mint=mint
    )]
    pub  vault:InterfaceAccount<'info,TokenAccount>,
    #[account(
        seeds=[b"marketplace",marketplace.name.as_bytes()],
        bump
    )]    
    pub marketplace:Account<'info,Marketplace>,
    #[account(
        mut,
        close=maker,
        seeds=[b"listing",marketplace.key().as_ref()],
        bump
    )]
    pub listing:Account<'info,Listing>,
    #[account(
        seeds=[b"rewards",marketplace.key().as_ref()],
        bump,
        mint::authority=marketplace,
        mint::decimals=6
    )]
    pub rewards:InterfaceAccount<'info,Mint>,
    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint=rewards,
        associated_token::authority=taker

    )]
    pub taker_rewards_ata:InterfaceAccount<'info,TokenAccount>,
    #[account(    
        mut,
        seeds = [b"treasury",marketplace.key().as_ref()],
        bump = marketplace.rewards_mint_bump,
    )]
    pub treasury:SystemAccount<'info>,
    pub system_program:Program<'info,System>,
    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>,
}
impl <'info>Purchase<'info> {
    pub fn purchase(&mut self)->Result<()>{ 
        self.sendsSol();
        self.purchase();
        self.rewardtaker();
        self.close()
    }
    //sends the sol from the vault to taker ata
    pub fn sendsSol(&mut self)->Result<()>{
        let program=self.token_program.to_account_info();
        let accoutns=Transfer{
            from:self.taker.to_account_info(),
            to:self.maker.to_account_info()
        };
        let ctx=CpiContext::new(program, accoutns);
        let marketplaceFees=(self.marketplace.fees as u64)*self.listing.price/10000;
        let amount=self.listing.price-marketplaceFees;

        transfer(ctx, amount);
        let program=self.token_program.to_account_info();

        let accounts=Transfer{
            from:self.taker.to_account_info(),
            to:self.treasury.to_account_info()
        };
        let ctx=CpiContext::new(program,accounts);
        transfer(ctx, marketplaceFees)
    }
    //transfer the nft
    pub fn takeNFt(&mut self)->Result<()>{
                let program=self.token_program.to_account_info();
        let accounts=TransferChecked{
            from:self.vault.to_account_info(),
            to:self.taker_ata.to_account_info(),
            authority:self.listing.to_account_info(),
            mint:self.mint.to_account_info()
        };
        let key=self.marketplace.key();
        let seeds=&[
            b"listing",
            key.as_ref(),
            &[self.listing.bump]
        ];
        let signer_seeeds=&[&seeds[..]];
        let ctx=CpiContext::new_with_signer(program, accounts, signer_seeeds);
        transfer_checked(ctx, 1, 0)
    }
    //reward the taker
    pub fn rewardtaker(&mut self)->Result<()>{
        let seeds=&[
            b"marketplace",
            self.marketplace.name.as_bytes(),
            &[self.marketplace.bump]
        ];
        let signer_seeds=&[&seeds[..]];
        let program=self.token_program.to_account_info();
        let ctx=CpiContext::new_with_signer(program, MintTo{
            mint:self.rewards.to_account_info(),
            authority:self.marketplace.to_account_info(),
            to:self.taker_rewards_ata.to_account_info()
        },signer_seeds);
        let amount=1e6 as u64;
        mint_to(ctx, amount)
        // todo!("amount not initialized")
    }
    //close the vault
    pub fn close(&mut self)->Result<()>{
           let program=self.token_program.to_account_info();
           let key=self.marketplace.key();
        let seeds=&[
            b"listing",
            key.as_ref(),
            &[self.listing.bump]
        ];
        let signer_seeeds=&[&seeds[..]];
        let accounts=CloseAccount{
            account:self.vault.to_account_info(),
            authority:self.listing.to_account_info(),
            destination:self.maker.to_account_info()
        };
        let ctx=CpiContext::new_with_signer(program, accounts, signer_seeeds);
        close_account(ctx)
        // todo!()
    }

}