use anyhow::Context;
use paint::Paintable;
use rustyline::{error::ReadlineError, DefaultEditor, Result};
use yansi::Color;

use crate::row::{Row, COLUMN_EMAIL_SIZE, COLUMN_USERNAME_SIZE, ROW_SIZE};

mod macro_helper;
mod paint;
mod row;

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

fn main() -> Result<()> {
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

        let source = [0; ROW_SIZE];
        let serialized_row = row.serialize(source);
        println!("serialized_row: {:?}", serialized_row);

        let deserialized_row = row.deserialize(serialized_row)?;
        println!("deserialized_row: {:?}", deserialized_row);

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
