use anchor_client::{
    anchor_lang::system_program,
    solana_sdk::{
        signature::Signature,
        pubkey::Pubkey,
    },
    Program,
};
use anyhow::Result;
use notes_system::state::SchoolMainAccount;

pub fn school_register(
    program: &Program,
    name: String,
    student_number: u64
) -> Result<()> {
    let (main_account, _bump): (Pubkey, u8) =
        Pubkey::find_program_address(&[b"Main Account"], &program.id());
    let main_account_data: SchoolMainAccount = program.account(main_account)?;
    let (school, _bump): (Pubkey, u8) =
        Pubkey::find_program_address(&[main_account_data.total_schools.to_be_bytes().as_ref()], &program.id());
    let tx: Signature = program
        .request()
        .accounts(notes_system::accounts::SchoolRegister {
            main_account,
            school,
            user: program.payer(),
            system_program: system_program::ID,
        })
        .args(notes_system::instruction::SchoolRegister {
            name,
            student_number
        })
        .send()?;
    println!("------------------------------------------------------------");
    println!("Tx: {}", tx);
    println!("------------------------------------------------------------");
    Ok(())
}