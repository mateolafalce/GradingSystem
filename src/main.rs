use anyhow::Result;
pub mod functions;
mod constants;
use constants::program;

fn main() -> Result<()> {
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]//cargo test school_main_account -- --show-output
    fn school_main_account() {
        functions::school_main_account::school_main_account(
            &program::program().unwrap(),
        )
        .unwrap();
    }
    #[test]//cargo test school_register -- --show-output
    fn school_register() {
        functions::school_register::school_register(
            &program::program().unwrap(),
            "Giga".to_string(),
            0
        )
        .unwrap();
    }
    #[test]//cargo test student_register -- --show-output
    fn student_register() {
        let school: Pubkey = Pubkey::from_str("BSpDLgixzye69jg8i3pwE5Y2vtfk1Hxv6aztMtTaNL6E").unwrap();
        functions::student_register::student_register(
            &program::program().unwrap(),
            "Mateo".to_string(),
            "Lafalce".to_string(),
            2,
            school
        )
        .unwrap();
    }
    #[test]//cargo test school_notes -- --show-output
    fn school_notes() {
        let school: Pubkey = Pubkey::from_str("69WHEGSYxyLKExZNW7rPmr4T1i3Te9N54VxWKafb6dDU").unwrap();
        let student: Pubkey = Pubkey::from_str("69WHEGSYxyLKExZNW7rPmr4T1i3Te9N54VxWKafb6dDU").unwrap();
        functions::school_notes::school_notes(
            &program::program().unwrap(),
            7,
            6,
            8,
            4,
            10,
            7,
            6,
            7,
            school,
            student
        )
        .unwrap();
    }
}