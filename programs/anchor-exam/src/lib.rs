use anchor_lang::prelude::*;

declare_id!("6VbBHa1qUhrbaaYTqDQbgxFSs4D1XDJ2G7PJm96uwVQo");

#[program]
pub mod anchor_exam {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
