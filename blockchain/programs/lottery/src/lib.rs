mod error;

use anchor_lang::prelude::*;

mod constant;

use crate::constant::MASTER_SEED;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod lottery {
    use super::*;

    pub fn init_master(ctx: Context<InitMaster>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMaster {
    #[account(
        init,
        payer=payer,
        space = 4+8,
        seeds=[MASTER_SEED.as_bytes()],
    )]
    pub master: Pubkey,
}
