use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
}; 
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn school_register(
    ctx: Context<SchoolRegister>,
    name: String,
    student_number: u64
) -> Result<()> {
    require!(name.len() <= 50, ErrorCode::LenghtError);
    let school: &mut Account<SchoolAccount> = &mut ctx.accounts.school;
    let (_pda, bump) = Pubkey::find_program_address(&[ctx.accounts.main_account.total_schools.to_be_bytes().as_ref()], ctx.program_id);
    school.bump_original = bump;
    school.admin = ctx.accounts.user.key();
    school.student_number = student_number;
    school.total_students = 0;
    school.seed = ctx.accounts.main_account.total_schools;
    let main_account: &mut Account<SchoolMainAccount> = &mut ctx.accounts.main_account;
    main_account.total_schools += 1;
    Ok(())
}

#[derive(Accounts)]
pub struct SchoolRegister<'info> {
    #[account(mut, seeds = [b"Main Account"], bump = main_account.bump_original)]
    pub main_account: Account<'info, SchoolMainAccount>,
    #[account(init, seeds = [main_account.total_schools.to_be_bytes().as_ref()], bump, payer = user, space = SchoolAccount::SIZE + 8)]
    pub school: Account<'info, SchoolAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}