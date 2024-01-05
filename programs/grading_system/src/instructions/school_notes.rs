use crate::state::accounts::*;
use anchor_lang::{prelude::*, solana_program::pubkey::Pubkey};

pub fn school_notes_(
    ctx: Context<SchoolNotes>,
    philosophy: u8,
    english: u8,
    biology: u8,
    chemistry: u8,
    mathematics: u8,
    deports: u8,
) -> Result<()> {
    let signer: Pubkey = ctx.accounts.user.key();
    let authority: Pubkey = ctx.accounts.school.admin.key();
    let student_number: &[u8] = &ctx.accounts.student.number.to_be_bytes();
    let trimester: &[u8] = &ctx.accounts.student.trimester.to_be_bytes();
    let program_id: &Pubkey = ctx.program_id;

    require_keys_eq!(signer, authority);

    let grades: &mut Account<GradesAccount> = &mut ctx.accounts.grades;
    let (_pda, bump) = Pubkey::find_program_address(&[&student_number, trimester], program_id);
    grades.set_bump_original(bump);
    grades.set_grades(
        philosophy,
        english,
        biology,
        chemistry,
        mathematics,
        deports,
    );

    let student: &mut Account<StudentAccount> = &mut ctx.accounts.student;
    student.add_trimester();
    Ok(())
}

#[derive(Accounts)]
pub struct SchoolNotes<'info> {
    #[account(mut, seeds = [&school.seed.to_be_bytes()], bump = school.bump_original)]
    pub school: Account<'info, SchoolAccount>,
    #[account(mut, seeds = [&student.number.to_be_bytes()], bump = student.bump_original)]
    pub student: Account<'info, StudentAccount>,
    #[account(
        init, seeds = [&student.number.to_be_bytes(), &student.trimester.to_be_bytes()], bump, payer = user, space = GradesAccount::SIZE
    )]
    pub grades: Account<'info, GradesAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
