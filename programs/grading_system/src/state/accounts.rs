use crate::utils::constants::{ANCHOR_BUFFER, MAX_LASTNAME, MAX_SCHOOL_NAME, MAX_STUDENT_NAME};
use anchor_lang::prelude::*;

#[account]
pub struct SchoolMainAccount {
    pub bump_original: u8,        // 1
    pub total_schools: u32,       // 4
    pub historical_students: u64, // 8
}

#[account]
pub struct SchoolAccount {
    pub bump_original: u8,   // 1
    pub admin: Pubkey,       // 32
    pub name: String,        // 4 + MAX_SCHOOL_NAME
    pub seed: u32,           // 4
    pub student_number: u64, // 8
    pub total_students: u64, // 8
}

#[account]
pub struct StudentAccount {
    pub bump_original: u8, // 1
    pub name: String,      // 4 + MAX_STUDENT_NAME
    pub lastname: String,  // 4 + MAX_LASTNAME
    pub number: u64,       // 8
    pub trimester: u64,    // 8
}

#[account]
pub struct GradesAccount {
    pub bump_original: u8, // 1
    pub philosophy: u8,    // 1
    pub english: u8,       // 1
    pub biology: u8,       // 1
    pub chemistry: u8,     // 1
    pub mathematics: u8,   // 1
    pub deports: u8,       // 1
}

impl StudentAccount {
    pub const SIZE: usize = 1 + 4 + MAX_STUDENT_NAME + 4 + MAX_LASTNAME + 8 + 8 + ANCHOR_BUFFER;

    pub fn set_bump_original(&mut self, bump: u8) {
        self.bump_original = bump;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_lastname(&mut self, lastname: String) {
        self.lastname = lastname
    }

    pub fn set_trimester(&mut self, trimester: u64) {
        self.trimester = trimester
    }

    pub fn set_number(&mut self, number: u64) {
        self.number = number;
    }

    pub fn add_trimester(&mut self) {
        self.trimester += 1;
    }
}

impl GradesAccount {
    pub const SIZE: usize = 1 + 1 + 1 + 1 + 1 + 1 + 1 + ANCHOR_BUFFER;

    pub fn set_bump_original(&mut self, bump: u8) {
        self.bump_original = bump;
    }

    pub fn set_grades(
        &mut self,
        philosophy: u8,
        english: u8,
        biology: u8,
        chemistry: u8,
        mathematics: u8,
        deports: u8,
    ) {
        self.philosophy = philosophy;
        self.english = english;
        self.biology = biology;
        self.chemistry = chemistry;
        self.mathematics = mathematics;
        self.deports = deports;
    }
}

impl SchoolAccount {
    pub const SIZE: usize = 1 + 32 + 4 + MAX_SCHOOL_NAME + 8 + 4 + 8 + ANCHOR_BUFFER;

    pub fn set_bump_original(&mut self, bump: u8) {
        self.bump_original = bump;
    }

    pub fn add_student_number(&mut self) {
        self.student_number += 1;
    }

    pub fn add_total_students(&mut self) {
        self.total_students += 1;
    }

    pub fn set_seed(&mut self, seed: u32) {
        self.seed = seed;
    }

    pub fn set_admin(&mut self, signer: Pubkey) {
        self.admin = signer;
    }

    pub fn set_student_number(&mut self, student_number: u64) {
        self.student_number = student_number;
    }

    pub fn init_total_students(&mut self) {
        self.total_students = 0;
    }
}

impl SchoolMainAccount {
    pub const SIZE: usize = 1 + 4 + 8 + ANCHOR_BUFFER;

    pub fn add_historical_students(&mut self) {
        self.historical_students += 1;
    }

    pub fn add_total_schools(&mut self) {
        self.total_schools += 1;
    }

    pub fn set_bump_original(&mut self, bump: u8) {
        self.bump_original = bump;
    }

    pub fn init_total_schools(&mut self) {
        self.total_schools = 0;
    }

    pub fn init_historical_students(&mut self) {
        self.historical_students = 0;
    }
}
