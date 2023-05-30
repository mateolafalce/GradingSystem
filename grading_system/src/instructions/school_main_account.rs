use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
};
use crate::state::accounts::*;

// Function to initialize the school main account
pub fn school_main_account(
    ctx: Context<InitSchoolMainAccount>
) -> Result<()> {
    let school_main_account: &mut Account<SchoolMainAccount> = &mut ctx.accounts.school_main_account;
    let (_pda, bump) = Pubkey::find_program_address(&[b"Main Account"], ctx.program_id);
    // Set the bump value for the school main account
    school_main_account.bump_original = bump;
    school_main_account.total_schools = 0;
    school_main_account.total_students = 0;
    Ok(())
}

#[derive(Accounts)]
pub struct InitSchoolMainAccount<'info> {
    #[account(init, seeds = [b"Main Account"], bump, payer = user, space = SchoolMainAccount::SIZE + 8)]
    pub school_main_account: Account<'info, SchoolMainAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
