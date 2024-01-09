use crate::{state::accounts::*, utils::constants::MAIN_ACCOUNT};
use anchor_lang::{prelude::*, solana_program::pubkey::Pubkey};

pub fn school_main_account_(ctx: Context<InitSchoolMainAccount>) -> Result<()> {
    let program_id: &Pubkey = ctx.program_id;
    let (pda, bump) = Pubkey::find_program_address(&[MAIN_ACCOUNT], program_id);
    require_keys_eq!(pda, ctx.accounts.main_account.key());
    let main_account: &mut Account<SchoolMainAccount> = &mut ctx.accounts.main_account;
    main_account.set_bump_original(bump);
    main_account.init_total_schools();
    main_account.init_historical_students();
    Ok(())
}

#[derive(Accounts)]
pub struct InitSchoolMainAccount<'info> {
    #[account(init, seeds = [MAIN_ACCOUNT], bump, payer = user, space = SchoolMainAccount::SIZE)]
    pub main_account: Account<'info, SchoolMainAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
