use anyhow::Context;
use paint::Paintable;
use rustyline::{error::ReadlineError, DefaultEditor, Result};
use std::fmt::Debug;
use yansi::Color;

mod paint;

const EXIT_SUCCESS: i32 = 0;
pub enum MetaCommandResult {
    Success,
    UnrecognizedCommand,
}

pub enum StatementCommandResult {
    Success(Statement),
    SyntaxError(String),
    Unrecognized,
}

pub enum Statement {
    Insert(Row),
    Select,
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Insert(_) => write!(f, "insert"),
            Self::Select => write!(f, "select"),
        }
    }
}

// this macro has been taken from this stack overflow post: https://stackoverflow.com/questions/61046063/how-to-get-the-size-of-a-struct-field-in-rust-without-instantiating-it
macro_rules! field_size {
    ($t:ident :: $field:ident) => {{
        let m = core::mem::MaybeUninit::<$t>::uninit();
        // According to https://doc.rust-lang.org/stable/std/ptr/macro.addr_of_mut.html#examples,
        // you can dereference an uninitialized MaybeUninit pointer in addr_of!
        // Raw pointer deref in const contexts is stabilized in 1.58:
        // https://github.com/rust-lang/rust/pull/89551
        let p = unsafe { core::ptr::addr_of!((*(&m as *const _ as *const $t)).$field) };

        const fn size_of_raw<T>(_: *const T) -> usize {
            core::mem::size_of::<T>()
        }
        size_of_raw(p)
    }};
}

const COLUMN_USERNAME_SIZE: usize = 32;
const COLUMN_EMAIL_SIZE: usize = 255;
const ID_SIZE: usize = field_size!(Row::id);
const USERNAME_SIZE: usize = field_size!(Row::username);
const EMAIL_SIZE: usize = field_size!(Row::email);
const ID_OFFSET: usize = 0;
const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;
const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;
const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;
#[derive(Debug)]
pub struct Row {
    pub id: u32,
    username: [u8; COLUMN_USERNAME_SIZE],
    email: [u8; COLUMN_EMAIL_SIZE],
}

impl Row {
    pub fn serialize(&self, destination: [u8; ROW_SIZE]) -> [u8; ROW_SIZE] {
        let mut serialized_row = destination;
        serialized_row[ID_OFFSET..ID_OFFSET + ID_SIZE].copy_from_slice(&self.id.to_le_bytes());

        serialized_row[USERNAME_OFFSET..USERNAME_OFFSET + USERNAME_SIZE]
            .copy_from_slice(&self.username);

        serialized_row[EMAIL_OFFSET..EMAIL_OFFSET + EMAIL_SIZE].copy_from_slice(&self.email);

        serialized_row
    }

    pub fn username(&self) -> String {
        let mut username = String::new();
        for byte in &self.username {
            if *byte == 0 {
                break;
            }
            username.push(*byte as char);
        }
        username
    }

    pub fn email(&self) -> String {
        let mut email = String::new();
        for byte in &self.email {
            if *byte == 0 {
                break;
            }
            email.push(*byte as char);
        }
        email
    }
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Row {{ id: {}, username: {}, email: {} }}",
            self.id,
            self.username(),
            self.email()
        )
    }
}

fn main() -> Result<()> {
    println!("size of val id:{}", ID_SIZE);
    println!("offset of val id:{}", ID_OFFSET);
    println!("size of val username:{}", USERNAME_SIZE);
    println!("offset of val username:{}", USERNAME_OFFSET);
    println!("size of val email:{}", EMAIL_SIZE);
    println!("offset of val email:{}", EMAIL_OFFSET);
    println!("size of val row:{}", ROW_SIZE);

    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline(&"lildb > ".paint(Color::Blue));
        match readline {
            Ok(line) => {
                if line.starts_with(".") {
                    match do_meta_command(&line) {
                        MetaCommandResult::Success => continue,
                        MetaCommandResult::UnrecognizedCommand => {
                            println!("Unrecognized command: {}", format!("\'{}\'", line).error());
                            continue;
                        }
                    }
                }

                if line.is_empty() {
                    continue;
                }

                match parsing_statement(&line) {
                    Ok(result) => match result {
                        StatementCommandResult::Success(statement) => {
                            println!(
                                "The command related to {} would be executed here.",
                                statement
                            );
                            println!("Executed.");
                            continue;
                        }
                        StatementCommandResult::SyntaxError(error) => {
                            println!("{}", error);
                            continue;
                        }
                        StatementCommandResult::Unrecognized => {
                            println!(
                                "Unrecognized keyword at start of {}",
                                format!("\'{}\'", line).error()
                            );
                            continue;
                        }
                    },
                    Err(error) => {
                        println!("{}", error);
                        continue;
                    }
                }
            }

            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}

fn parsing_statement(line: &str) -> anyhow::Result<StatementCommandResult> {
    if line.contains("insert") {
        let input = line.split_whitespace().collect::<Vec<&str>>();
        if input.len() != 4 {
            return Ok(StatementCommandResult::SyntaxError(format!(
                "{}",
                "Insert command takes exactly 3 args.".error()
            )));
        }

        if input[2].len() > COLUMN_USERNAME_SIZE {
            return Ok(StatementCommandResult::SyntaxError(format!(
                "{}",
                "Username is too long.".error()
            )));
        }

        if input[3].len() > COLUMN_EMAIL_SIZE {
            return Ok(StatementCommandResult::SyntaxError(format!(
                "{}",
                "Email is too long.".error()
            )));
        }

        let mut username: [u8; COLUMN_USERNAME_SIZE] = [0; COLUMN_USERNAME_SIZE];
        let mut email: [u8; COLUMN_EMAIL_SIZE] = [0; COLUMN_EMAIL_SIZE];

        username[..input[2].len()].copy_from_slice(input[2].as_bytes());
        email[..input[3].len()].copy_from_slice(input[3].as_bytes());

        let row = Row {
            id: input[1].parse().context(format!(
                "{}",
                "The row id is of type 'u32' for a User".error()
            ))?,
            username,
            email,
        };

        let serialized_row = row.serialize([0; ROW_SIZE]);
        println!("serialized_row: {:?}", serialized_row);

        println!(
            "{}",
            format!("Inserting {} in the database...", row).paint(Color::Cyan)
        );

        return Ok(StatementCommandResult::Success(Statement::Insert(row)));
    }

    if line.contains("select") {
        return Ok(StatementCommandResult::Success(Statement::Select));
    }

    Ok(StatementCommandResult::Unrecognized)
}

fn do_meta_command(line: &str) -> MetaCommandResult {
    if line == ".exit" {
        println!("{}", "Exiting...".paint(Color::Green));
        std::process::exit(EXIT_SUCCESS);
    }

    MetaCommandResult::UnrecognizedCommand
}
