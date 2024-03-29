<div align="center">

![GradingSystem](GradingSystem.png)

## Grading System

---

![GitHub Repo stars](https://img.shields.io/github/stars/mateolafalce/GradingSystem?color=greent&style=for-the-badge)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/mateolafalce/GradingSystem?color=greent&style=for-the-badge)
![GitHub forks](https://img.shields.io/github/forks/mateolafalce/GradingSystem?color=greent&style=for-the-badge)

![GitHub](https://img.shields.io/github/license/mateolafalce/GradingSystem?color=greent&style=for-the-badge)
![GitHub pull requests](https://img.shields.io/github/issues-pr/mateolafalce/RustyFltkIDE?color=greent&style=for-the-badge)

![GitHub closed pull requests](https://img.shields.io/github/issues-pr-closed-raw/mateolafalce/GradingSystem?color=greent&style=for-the-badge)
![GitHub closed issues](https://img.shields.io/github/issues-closed/mateolafalce/GradingSystem?color=greent&style=for-the-badge)

</div>

---

The GradingSystem repository contains the source code of a decentralized application based on the Solana blockchain, which allows the storage of school and university grades in a secure and decentralized manner. The application was designed to offer a secure and reliable alternative to traditional registration systems, in which data is centralized and susceptible to fraud and manipulation.

The app uses Solana's blockchain technology to publicly and securely store scores, allowing anyone to access them and verify their authenticity without the need for intermediaries. Furthermore, the app uses cryptography to protect the privacy of student data, only allowing authorized users to access it. The application source code is available in this Github repository under the Apache 2.0 license, and is available for anyone to review, modify, and contribute to the development of the project.

---

<details>
<summary>Init a the main account 🏢</summary>

<br>

```rust
pub fn school_main_account(
    ctx: Context<InitSchoolMainAccount>
) -> Result<()> {
    let school_main_account: &mut Account<SchoolMainAccount> = &mut ctx.accounts.school_main_account;
    let (_pda, bump) = Pubkey::find_program_address(&[b"Main Account"], ctx.program_id);
    school_main_account.bump_original = bump;
    school_main_account.total_schools = 0;
    school_main_account.total_students = 0;
    Ok(())
}

#[derive(Accounts)]
pub struct InitSchoolMainAccount<'info> {
    #[account(init, seeds = [b"Main Account"], bump, payer = user, space = SchoolMainAccount::SIZE + 8)]
    pub school_main_account: Account<'info, SchoolMainAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

The school_main_account function is an initialization function for an account of type SchoolMainAccount in Solana. This account is used in the Solana blockchain to store information related to schools and students.

The function takes a Context<InitSchoolMainAccount> as an argument and returns a Result<()>. The context contains the information needed to initialize the account, including the account itself (school_main_account), a signer (user), and the Solana system program (system_program).

In the function implementation, you get a mutable reference to the school_main_account account. Then, a program public address (pda) and bump value are generated using the Pubkey::find_program_address() function. The public address is a unique address generated by the program and the increment value is a randomly generated number used to prevent address collisions.

Next, the initial values ​​of the account fields school_main_account are set. The bump_original field is set to the bump generated value, which is used to generate the public address. The total_schools and total_students fields are set to zero.

Finally, an Ok(()) result is returned indicating that the account initialization was successful.

</details>

---

<details>
<summary>Register an school/university 📚 </summary>

<br>

```rust
pub fn school_register(
    ctx: Context<SchoolRegister>,
    name: String,
    student_number: u64
) -> Result<()> {
    require!(name.len() <= 50, ErrorCode::LenghtError);
    let school: &mut Account<SchoolAccount> = &mut ctx.accounts.school;
    let (_pda, bump) = Pubkey::find_program_address(&[ctx.accounts.main_account.total_schools.to_be_bytes().as_ref()], ctx.program_id);
    school.bump_original = bump;
    school.admin = ctx.accounts.user.key();
    school.student_number = student_number;
    school.total_students = 0;
    school.seed = ctx.accounts.main_account.total_schools;
    let main_account: &mut Account<SchoolMainAccount> = &mut ctx.accounts.main_account;
    main_account.total_schools += 1;
    Ok(())
}

#[derive(Accounts)]
pub struct SchoolRegister<'info> {
    #[account(mut, seeds = [b"Main Account"], bump = main_account.bump_original)]
    pub main_account: Account<'info, SchoolMainAccount>,
    #[account(init, seeds = [main_account.total_schools.to_be_bytes().as_ref()], bump, payer = user, space = SchoolAccount::SIZE + 8)]
    pub school: Account<'info, SchoolAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

The function takes as input the ctx context, which includes a series of accounts that must be used in the operation. You are also expected to provide the name of the school and the number of students it has.

The code first checks that the school name does not exceed 50 characters, and then uses the total number of schools registered to the main account main_account to generate a bump for the school account. The school account is then initialized with the provided values, such as the school name, number of students, seed, and bump.

After the school account is created, the function updates the main account main_account to reflect the fact that a new school has been registered. Finally, the function returns an Ok(()) result to indicate that the operation has completed successfully.

The SchoolRegister structure defines the requirements for the accounts that must be provided when calling the school_register function. The main_account is an account that keeps track of all schools registered on the blockchain. The school account is the school account that will be created during the execution of the function. The user account is the account of the user making the function call, and the system_program account is a system account required by the function.

</details>

---

<details>
<summary>Register an student 🤓</summary>

<br>

```rust
pub fn student_register(
    ctx: Context<StudentRegister>,
    name: String,
    lastname: String,
    trimester: u8,
) -> Result<()> {
    require!(trimester > 0, ErrorCode::TrimesterError);
    require!(name.len() <= 50, ErrorCode::LenghtError);
    require!(lastname.len() <= 50, ErrorCode::LenghtError);
    require!(ctx.accounts.user.key() == ctx.accounts.school.admin.key(), ErrorCode::AuthorityError);
    let student: &mut Account<StudentAccount> = &mut ctx.accounts.student;
    let (_pda, bump) = Pubkey::find_program_address(&[ctx.accounts.school.student_number.to_be_bytes().as_ref()], ctx.program_id);
    student.bump_original = bump;
    student.name = name;
    student.lastname = lastname;
    student.trimester = trimester;
    student.number = ctx.accounts.school.student_number;
    let school: &mut Account<SchoolAccount> = &mut ctx.accounts.school;
    school.student_number += 1;
    school.total_students += 1;
    let main_account: &mut Account<SchoolMainAccount> = &mut ctx.accounts.main_account;
    main_account.total_students += 1;
    Ok(())
}

#[derive(Accounts)]
pub struct StudentRegister<'info> {
    #[account(mut, seeds = [b"Main Account"], bump = main_account.bump_original)]
    pub main_account: Account<'info, SchoolMainAccount>,
    #[account(mut, seeds = [school.seed.to_be_bytes().as_ref()], bump = school.bump_original)]
    pub school: Account<'info, SchoolAccount>,
    #[account(init, seeds = [school.student_number.to_be_bytes().as_ref()], bump, payer = user, space = StudentAccount::SIZE + 8)]
    pub student: Account<'info, StudentAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

This function takes as arguments the context, which contains information relevant to the transaction, such as the student's account to register, as well as the student's first name, last name, and term.

The function first validates that the quarter is greater than zero, the first and last name do not exceed 50 characters, and that the password of the user who performs the transaction is the same as that of the school administrator. It then uses the student's account to store their information and generates a unique program address using the student number and program key. Then, update the number of students and the total number of students registered in the school accounts and the main school account. Finally, it returns a success result.

The StudentRegister structure is used to define the accounts that are used in the function. In particular, the feature requires access to the school's account, the account of the student being registered, the school's main account, and the account of the user making the transaction. Each account is defined using the #[account] macro and additional information is provided, such as seeds and bump, which are used to generate the unique program address.

</details>

---

<details>
<summary>Record grades 📝</summary>

<br>

```rust
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
    require!(ctx.accounts.user.key() == ctx.accounts.school.admin.key(), ErrorCode::AuthorityError);
    let student: &mut Account<StudentAccount> = &mut ctx.accounts.student;
    let notes: &mut Account<NotesAccount> = &mut ctx.accounts.notes;
    notes.philosophy.push(philosophy);
    notes.english.push(english);
    notes.biology.push(biology);
    notes.physical.push(physical);
    notes.chemistry.push(chemistry);
    notes.mathematics.push(mathematics);
    notes.work_and_citizenship.push(work_and_citizenship);
    notes.deports.push(deports);
    student.trimester += 1;
    Ok(())
}

#[derive(Accounts)]
pub struct SchoolNotes<'info> {
    #[account(mut, seeds = [school.seed.to_be_bytes().as_ref()], bump = school.bump_original)]
    pub school: Account<'info, SchoolAccount>,
    #[account(mut, seeds = [student.number.to_be_bytes().as_ref()], bump = student.bump_original)]
    pub student: Account<'info, StudentAccount>,
    #[account(init, seeds = [student.number.to_be_bytes().as_ref(), student.trimester.to_be_bytes().as_ref()], bump, payer = user, space = NotesAccount::SIZE + 8)]
    pub notes: Account<'info, NotesAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

The function takes as arguments notes in different subjects, such as philosophy, English, biology, physics, chemistry, mathematics, work and citizenship, and sports. The function adds these notes to a student's note record in Solana's account. The function requires the user password to be the same as the school administrator password in order to run. The function then adds the grades in the different subjects to a student's grade count on the Solana blockchain and increases the student's quarter number by one.

The function is designed to be used with the SchoolNotes structure that contains a school account, a student account, a notes account, and other variables necessary for its execution.

The SchoolNotes structure is decorated with Rust's Accounts attribute, which specifies that the fields within the structure represent Solana accounts that are used in the function. The SchoolNotes structure has four fields, school, student, notes, user, and system_program. The school field represents the school account, and is labeled mutable and has the seeds required to generate its address on the Solana blockchain. The student field represents the student's account, and is also labeled mutable and has the required seeds to generate their address on the Solana blockchain.

The notes field represents the student's notes account, which is initialized with the seeds required to generate their address on the Solana blockchain, payer, account size, and is tagged as mutable. The user field represents the digital signature of the user calling the function and is also labeled mutable.

Finally, the system_program field represents the system program used to execute the function on the Solana blockchain.

</details>

---
