use crate::{
    state::accounts::*,
    utils::constants::{MAIN_ACCOUNT, MAX_LASTNAME, MAX_STUDENT_NAME},
};
use anchor_lang::{prelude::*, solana_program::pubkey::Pubkey};

pub fn student_register_(
    ctx: Context<StudentRegister>,
    name: String,
    lastname: String,
    trimester: u64,
) -> Result<()> {
    let signer: Pubkey = ctx.accounts.user.key();
    let school_authority: Pubkey = ctx.accounts.school.admin.key();
    let student_number: u64 = ctx.accounts.school.student_number;
    let program_id: &Pubkey = ctx.program_id;

    require_gt!(trimester, 0);
    require_gte!(MAX_STUDENT_NAME, name.len());
    require_gte!(MAX_LASTNAME, lastname.len());
    require_keys_eq!(signer, school_authority);

    let student: &mut Account<StudentAccount> = &mut ctx.accounts.student;
    let (_pda, bump) = Pubkey::find_program_address(&[&student_number.to_be_bytes()], program_id);
    student.set_bump_original(bump);
    student.set_name(name);
    student.set_lastname(lastname);
    student.set_trimester(trimester);
    student.set_number(student_number);

    let school: &mut Account<SchoolAccount> = &mut ctx.accounts.school;
    school.add_student_number();
    school.add_total_students();

    let main_account: &mut Account<SchoolMainAccount> = &mut ctx.accounts.main_account;
    main_account.add_historical_students();
    Ok(())
}

#[derive(Accounts)]
pub struct StudentRegister<'info> {
    #[account(mut, seeds = [MAIN_ACCOUNT], bump = main_account.bump_original)]
    pub main_account: Account<'info, SchoolMainAccount>,
    #[account(mut, seeds = [&school.seed.to_be_bytes()], bump = school.bump_original)]
    pub school: Account<'info, SchoolAccount>,
    #[account(
        init, seeds = [&school.student_number.to_be_bytes()], bump, payer = user, space = StudentAccount::SIZE
    )]
    pub student: Account<'info, StudentAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
