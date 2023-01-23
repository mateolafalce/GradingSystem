use anchor_lang::prelude::*;

#[account]
pub struct SchoolMainAccount { 
    pub bump_original: u8,         // 1
    pub total_schools: u64,        // 8
    pub total_students: u64,       // 8
}

#[account]
pub struct SchoolAccount { 
    pub bump_original: u8,          // 1
    pub admin: Pubkey,              // 32
    pub name: String,               // 4 + 50
    pub seed: u64,                  // 8
    pub student_number: u64,        // 8
    pub total_students: u64,        // 8
}

#[account]
pub struct StudentAccount { 
    pub bump_original: u8,              // 1
    pub name: String,                   // 4 + 50
    pub lastname: String,               // 4 + 50
    pub number: u64,                    // 8
    pub trimester: u8,                  // 1
}

#[account]
pub struct NotesAccount { 
    pub bump_original: u8,              // 1
    pub philosophy: Vec<u8>,            // 4 + 1
    pub english: Vec<u8>,               // 4 + 1
    pub biology: Vec<u8>,               // 4 + 1
    pub physical: Vec<u8>,              // 4 + 1
    pub chemistry: Vec<u8>,             // 4 + 1
    pub mathematics: Vec<u8>,           // 4 + 1
    pub work_and_citizenship: Vec<u8>,  // 4 + 1
    pub deports: Vec<u8>,               // 4 + 1
}

impl StudentAccount {
    pub const SIZE: usize =  1 + 4 + 50 + 4 + 50 + 8 + 1;
}

impl NotesAccount {
    pub const SIZE: usize = 1 + 32 + 8;
}

impl SchoolAccount {
    pub const SIZE: usize =  1 + 32 + 4 + 50 + 8 + 8 + 8;
}

impl SchoolMainAccount {
    pub const SIZE: usize =  1 + 8 + 8;
}
