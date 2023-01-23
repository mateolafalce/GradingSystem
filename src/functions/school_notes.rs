use anchor_client::{
    anchor_lang::system_program,
    solana_sdk::{
        signature::Signature,
        pubkey::Pubkey,
    },
    Program,
};
use anyhow::Result;
use notes_system::state::StudentAccount;

pub fn school_notes(
    program: &Program,
    philosophy: u8,
    english: u8,
    biology: u8,
    physical: u8,
    chemistry: u8,
    mathematics: u8,
    work_and_citizenship: u8,
    deports: u8,
    school: Pubkey,
    student: Pubkey
) -> Result<()> {
    let student_data: StudentAccount = program.account(student)?;
    let (notes, _bump): (Pubkey, u8) =
        Pubkey::find_program_address(&[student_data.number.to_be_bytes().as_ref(), student_data.trimester.to_be_bytes().as_ref()], &program.id());
    let tx: Signature = program
        .request()
        .accounts(notes_system::accounts::SchoolNotes {
            school,
            student,
            notes,
            user: program.payer(),
            system_program: system_program::ID,
        })
        .args(notes_system::instruction::SchoolNotes {
            philosophy,
            english,
            biology,
            physical,
            chemistry,
            mathematics,
            work_and_citizenship,
            deports
        })
        .send()?;
    println!("------------------------------------------------------------");
    println!("Tx: {}", tx);
    println!("------------------------------------------------------------");
    Ok(())
}