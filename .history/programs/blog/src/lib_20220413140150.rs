use anchor_lang::prelude::*;
use std::str::from_utf8;


declare_id!("4zRip5KNv4AXiCgdVQZdmeAFaVuvcmvd7dxkopAvP89N");

#[program]
pub mod blog {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let blog_acc = &mut ctx.accounts.blog_account;
        blog_acc.authority = *ctx.accounts.authority.key;
        Ok(())
    }

    pub fn make_post(ctx: Context<MakePost>) -> Result<()> {
        //post
        Ok(())
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
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct MakePost<'info> {
    #[account(mut, has_one = authority)]
    pub blog_account: Account<'info,BlogAccount >,
    pub authority: Signer<'info>
}

// Where to store elements of the blog 
#[account]
pub struct BlogAccount {
    pub post: u64,
    pub authority: Pubkey
}
