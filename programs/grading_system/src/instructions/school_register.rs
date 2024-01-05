use crate::{
    state::accounts::*,
    utils::constants::{MAIN_ACCOUNT, MAX_SCHOOL_NAME},
};
use anchor_lang::{prelude::*, solana_program::pubkey::Pubkey};

pub fn school_register_(
    ctx: Context<SchoolRegister>,
    name: String,
    student_number: u64,
) -> Result<()> {
    let signer: Pubkey = ctx.accounts.user.key();
    let program_id: &Pubkey = ctx.program_id;
    let total_schools_bytes: &[u8] = &ctx.accounts.main_account.total_schools.to_be_bytes();
    let total_schools: u32 = ctx.accounts.main_account.total_schools;

    require_gte!(MAX_SCHOOL_NAME, name.len());

    let school: &mut Account<SchoolAccount> = &mut ctx.accounts.school;
    let (_pda, bump) = Pubkey::find_program_address(&[total_schools_bytes], program_id);
    school.set_bump_original(bump);
    school.set_admin(signer);
    school.set_student_number(student_number);
    school.init_total_students();
    school.set_seed(total_schools);

    let main_account: &mut Account<SchoolMainAccount> = &mut ctx.accounts.main_account;
    main_account.add_total_schools();
    Ok(())
}

#[derive(Accounts)]
pub struct SchoolRegister<'info> {
    #[account(mut, seeds = [MAIN_ACCOUNT], bump = main_account.bump_original)]
    pub main_account: Account<'info, SchoolMainAccount>,
    #[account(
        init, seeds = [&main_account.total_schools.to_be_bytes()], bump, payer = user, space = SchoolAccount::SIZE
    )]
    pub school: Account<'info, SchoolAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
