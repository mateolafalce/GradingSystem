use anchor_client::{
    anchor_lang::system_program,
    solana_sdk::{
        signature::Signature,
        pubkey::Pubkey,
    },
    Program,
};
use anyhow::Result;

pub fn school_main_account(
    program: &Program,
) -> Result<()> {
    let (school_main_account, _bump): (Pubkey, u8) =
        Pubkey::find_program_address(&[b"Main Account"], &program.id());
    let tx: Signature = program
        .request()
        .accounts(notes_system::accounts::InitSchoolMainAccount {
            school_main_account,
            user: program.payer(),
            system_program: system_program::ID,
        })
        .args(notes_system::instruction::SchoolMainAccount {})
        .send()?;
    println!("------------------------------------------------------------");
    println!("Tx: {}", tx);
    println!("------------------------------------------------------------");
    Ok(())
}