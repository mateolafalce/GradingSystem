use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
};
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn student_register(
    ctx: Context<StudentRegister>,
    name: String,
    lastname: String,
    trimester: u8,
) -> Result<()> {
    // Check if trimester is greater than 0
    require!(trimester > 0, ErrorCode::TrimesterError);
    // Check if name length is less than or equal to 50 characters
    require!(name.len() <= 50, ErrorCode::LengthError);
    // Check if lastname length is less than or equal to 50 characters
    require!(lastname.len() <= 50, ErrorCode::LengthError);
    // Check if the caller is the admin of the school
    require!(ctx.accounts.user.key() == ctx.accounts.school.admin.key(), ErrorCode::AuthorityError);
    let student: &mut Account<StudentAccount> = &mut ctx.accounts.student;D
    let (_pda, bump) = Pubkey::find_program_address(&[ctx.accounts.school.student_number.to_be_bytes().as_ref()], ctx.program_id);
    student.bump_original = bump;
    student.name = name;
    student.lastname = lastname;
    student.trimester = trimester;
    student.number = ctx.accounts.school.student_number;
    let school: &mut Account<SchoolAccount> = &mut ctx.accounts.school;
    // Increment the student number in the school account
    school.student_number += 1;
    school.total_students += 1;
    let main_account: &mut Account<SchoolMainAccount> = &mut ctx.accounts.main_account;
    // Increment the total number of students in the main account
    main_account.total_students += 1;
    Ok(())
}

#[derive(Accounts)]
pub struct StudentRegister<'info> {
    // Main account, mutable and initialized with seeds and bump value
    #[account(mut, seeds = [b"Main Account"], bump = main_account.bump_original)]
    pub main_account: Account<'info, SchoolMainAccount>,
    // School account, mutable and initialized with seeds and bump value
    #[account(mut, seeds = [school.seed.to_be_bytes().as_ref()], bump = school.bump_original)]
    pub school: Account<'info, SchoolAccount>,
    // Student account, initialized with seeds and bump value, and payer set to user
    #[account(init, seeds = [school.student_number.to_be_bytes().as_ref()], bump, payer = user, space = StudentAccount::SIZE + 8)]
    pub student: Account<'info, StudentAccount>,
    // Signer account
    #[account(mut)]
    pub user: Signer<'info>,
    // System program account
    pub system_program: Program<'info, System>,
}
