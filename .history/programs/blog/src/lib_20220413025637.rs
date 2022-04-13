use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod blog {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        //ctx.accounts
        Ok(())
    }
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer= Signer)]
    pub blog_account: Account<'info, BlogAccount>;
    pub authority: Signer;
}

// Where to store elements of the blog 
#[account]
pub struct BlogAccount {
    pub post: u64,
    pub authority: Pubkey
}
