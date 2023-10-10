use anchor_lang::prelude::*;
use crate::state::*;
use anchor_spl::token::{Mint, TokenAccount, Token};
use anchor_spl::associated_token::AssociatedToken;

#[derive(Accounts)]
#[instruction(_event_id: u64)]
pub struct EventCreationContext<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + EventAccount::MAX_SIZE,
        seeds = [b"event-data".as_ref(), _event_id.to_le_bytes().as_ref()], 
        bump
    )]
    pub event_account: Box<Account<'info, EventAccount>>,
   

    #[account(mut)]
    pub token_mint: Account<'info, Mint>,


    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<EventCreationContext>, _event_id: u64, _supply: u64, _price:u64, _date: u64) -> Result<()> {

    let event_account = &mut ctx.accounts.event_account;
    let authority_clone = ctx.accounts.authority.to_account_info().key();
    let token_mint = ctx.accounts.token_mint.to_account_info().key();

    // pub event_id: u64,
    // pub supply: u64,
    // pub date: u64, //epoch timestamps
    // pub price: u64, // in lamports
    // pub event_manager: Pubkey,
    // pub token: Pubkey,
    // pub bump: u8, // for calculating pda

    //* amount and supply */

    // Initialize the escrow account fields
    event_account.event_id = _event_id;
    event_account.supply = _supply;
    event_account.event_manager = authority_clone.key();
    event_account.price = _price;
    event_account.token = token_mint.key();
    event_account.date = _date;
    event_account.bump = *ctx.bumps.get("event_account").unwrap();

    Ok(())
}