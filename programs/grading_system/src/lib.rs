use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod state;
pub mod utils;

use instructions::{
    school_main_account::school_main_account_, school_notes::school_notes_,
    school_register::school_register_, student_register::student_register_,
};

declare_id!("5GZTR4UtisYrZ8aUN3gsUisMESbMKBMgdbkZezzKouQs");

#[program]
pub mod grading_system {
    use super::*;
    // global account for stats
    pub fn school_main_account(ctx: Context<InitSchoolMainAccount>) -> Result<()> {
        school_main_account_(ctx)
    }
    // account for new institutions
    pub fn school_register(
        ctx: Context<SchoolRegister>,
        name: String,
        student_number: u64,
    ) -> Result<()> {
        school_register_(ctx, name, student_number)
    }
    // for register of students
    pub fn student_register(
        ctx: Context<StudentRegister>,
        name: String,
        lastname: String,
        trimester: u64,
    ) -> Result<()> {
        student_register_(ctx, name, lastname, trimester)
    }
    // for notes!
    pub fn school_notes(
        ctx: Context<SchoolNotes>,
        philosophy: u8,
        english: u8,
        biology: u8,
        chemistry: u8,
        mathematics: u8,
        deports: u8,
    ) -> Result<()> {
        school_notes_(
            ctx,
            philosophy,
            english,
            biology,
            chemistry,
            mathematics,
            deports,
        )
    }
}
