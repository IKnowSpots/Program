use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod iknowspots {
    use crate::instruction::EventCreation;

    use super::*;

    pub fn initialize(ctx: Context<InitializeContext>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn event_creation(ctx: Context<EventCreationContext>, _event_id: u64) -> Result<()> {
        event_creation::handler(ctx, _event_id)
    }

}







