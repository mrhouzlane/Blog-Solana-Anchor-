use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod blog {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}


// Where the storage happens 
#[derive(Accounts)]

#[account]
pub struct MyAccount {
    pub data: u64,
}
pub struct Initialize {}
