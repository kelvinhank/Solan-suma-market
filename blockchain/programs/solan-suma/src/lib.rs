use anchor_lang::prelude::*;

declare_id!("CCVuhWUvUrdyAZA2AVYpVGZw21J1jVSyPVD4kzGH1oGJ");

#[program]
pub mod solan_suma {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
