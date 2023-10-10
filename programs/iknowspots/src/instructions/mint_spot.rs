use anchor_lang::prelude::*;
use crate::state::*;
use anchor_spl::token::{Mint, TokenAccount, Token};
use anchor_spl::associated_token::AssociatedToken;

#[derive(Accounts)]
#[instruction(_event_id: u64, _event_bump: u8)]
pub struct MintSpotContext<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        mut,
        constraint = token_mint.key() == event_account.token,
        seeds = [b"event-data".as_ref(), _event_id.to_le_bytes().as_ref()], 
        bump = event_account.bump
    )]
    pub event_account: Box<Account<'info, EventAccount>>,
   
    #[account(
        mut,
        seeds = [b"event-asset".as_ref(), _event_id.to_le_bytes().as_ref()], 
        bump = _event_bump
    )]
    pub event_token_account: Box<Account<'info, TokenAccount>>,


    #[account(mut)]
    pub token_mint: Account<'info, Mint>,

    #[account(mut, constraint = token_ata_sender.mint == token_mint.key(), constraint = token_ata_sender.owner == authority.key())]
    pub token_ata_sender: Box<Account<'info, TokenAccount>>,

    

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<MintSpotContext>, _event_id: u64, _event_bump: u8, _supply: u64, _price:u64, _date: u64) -> Result<()> {

    let event_account = &mut ctx.accounts.event_account;
    let authority_clone = ctx.accounts.authority.to_account_info().key();
    let token_mint = ctx.accounts.token_mint.to_account_info().key();

    event_account.minted += 1;

    // Create a transfer instruction to move tokens from the token_ata_sender account to the escrow_token_account
    let transfer_instruction = anchor_spl::token::Transfer {
        from: ctx.accounts.token_ata_sender.to_account_info(),
        to: ctx.accounts.event_token_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };

    // Create a CPI (Cross-Program Invocation) context for calling the token program's transfer instruction
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        transfer_instruction,
    );

    // Call the token program's transfer instruction using the CPI context and transfer the specified amount of tokens
    anchor_spl::token::transfer(cpi_ctx, event_account.price)?;

    Ok(())
}