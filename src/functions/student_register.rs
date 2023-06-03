use anchor_client::{
    anchor_lang::system_program,
    solana_sdk::{
        signature::Signature,
        pubkey::Pubkey,
    },
    Program,
};
use anyhow::Result;
use notes_system::state::SchoolAccount;

pub fn student_register(
    program: &Program,
    name: String,
    lastname: String,
    trimester: u8,
    school: Pubkey
) -> Result<()> {
    // Generate a unique main account key based on the program ID
    let (main_account, _bump): (Pubkey, u8) =
        Pubkey::find_program_address(&[b"Main Account"], &program.id());
    // Fetch the school account associated with the provided pubkey
    let school_account: SchoolAccount = program.account(school)?;
    // Generate a unique student account key based on the school account's student number and program ID
    let (student, _bump): (Pubkey, u8) =
        Pubkey::find_program_address(&[school_account.student_number.to_be_bytes().as_ref()], &program.id());
    // Invoke the program's 'StudentRegister' instruction by creating and sending a transaction
    let tx: Signature = program
        .request()
        .accounts(notes_system::accounts::StudentRegister {
            main_account,
            school,
            student,
            user: program.payer(),
            system_program: system_program::ID,
        })
        .args(notes_system::instruction::StudentRegister {
            name,
            lastname,
            trimester
        })
        .send()?;
    // Print transaction information for debugging purposes
    println!("------------------------------------------------------------");
    println!("Tx: {}", tx);
    println!("------------------------------------------------------------");

    Ok(())
}
