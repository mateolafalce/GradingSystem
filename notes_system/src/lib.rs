use anchor_lang::prelude::*;
use instructions::*;
use crate::errors::ErrorCode;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("69WHEGSYxyLKExZNW7rPmr4T1i3Te9N54VxWKafb6dDU");

#[program]
pub mod notes_system {
    use super::*;
    pub fn school_main_account(ctx: Context<InitSchoolMainAccount>) -> Result<()> {
        instructions::school_main_account::school_main_account(ctx)
    }
    pub fn school_register(
        ctx: Context<SchoolRegister>,
        name: String,
        student_number: u64
    ) -> Result<()> {
        instructions::school_register::school_register(
            ctx,
            name,
            student_number
        )
    }
    pub fn student_register(
        ctx: Context<StudentRegister>,
        name: String,
        lastname: String,
        trimester: u8
    ) -> Result<()> {
        instructions::student_register::student_register(
            ctx,
            name,
            lastname,
            trimester
        )
    }
    pub fn school_notes(
        ctx: Context<SchoolNotes>,
        philosophy: u8,
        english: u8,
        biology: u8,
        physical: u8,
        chemistry: u8,
        mathematics: u8,
        work_and_citizenship: u8,
        deports: u8,
    ) -> Result<()> {
        instructions::school_notes::school_notes(
            ctx,
            philosophy,
            english,
            biology,
            physical,
            chemistry,
            mathematics,
            work_and_citizenship,
            deports,
        )
    }
}