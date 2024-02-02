use anchor_lang::prelude::*;

declare_id!("6uYoToLfeWymrn3hDL5kL6bLM74dS8dPzPbMdHavjgCz");

#[program]
pub mod puppet {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
