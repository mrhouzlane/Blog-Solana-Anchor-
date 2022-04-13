use anchor_lang::prelude::*;

declare_id!("4zRip5KNv4AXiCgdVQZdmeAFaVuvcmvd7dxkopAvP89N");

#[program]
pub mod blog {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult<()> {
        let blog_account = &mut ctx.accounts.blog_account;
        blog_acc.authority = *ctx.accounts.authority.key;
        Ok(())
    }

    pub fn make_post(ctx: Context<MakePost>) -> ProgramResult<()> {

    }
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer= authority,
        space= 8 // account discriminator 
        + 32 // pubkey
        + 566 // posts max length 
    )]
    pub blog_account: Account<'info, BlogAccount>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>
}

pub struct MakePost<'info> {
    pub blog_account: Account<'info,BlogAccount >
}

// Where to store elements of the blog 
#[account]
pub struct BlogAccount {
    pub post: u64,
    pub authority: Pubkey
}
