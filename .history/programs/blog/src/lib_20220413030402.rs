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
pub struct Initialize {
    #[account(
        init,
        payer= authority,
        space= 8 // account discriminator 
        + 32 // pubkey
        + 566 // posts max length 
    )]
    pub blog_account: Account<'info, BlogAccount>,
    //#[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>
}

// Where to store elements of the blog 
#[account]
pub struct BlogAccount {
    pub post: u64,
    pub authority: Pubkey
}
