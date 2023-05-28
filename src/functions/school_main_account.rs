pub fn school_main_account(
    program: &anchor_client::Program,
) -> anyhow::Result<()> {
    let (school_main_account, _bump): (solana_sdk::pubkey::Pubkey, u8) =
        Pubkey::find_program_address(&[b"Main Account"], &program.id());
    let tx: solana_sdk::signature::SignatureSignature = program
        .request()
        .accounts(notes_system::accounts::InitSchoolMainAccount {
            school_main_account,
            user: program.payer(),
            system_program: anchor_lang::system_program::system_program::ID,
        })
        .args(notes_system::instruction::SchoolMainAccount {})
        .send()?;
    println!("------------------------------------------------------------");
    println!("Tx: {}", tx);
    println!("------------------------------------------------------------");
    Ok(())
}
