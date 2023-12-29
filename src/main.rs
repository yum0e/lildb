use rustyline::{error::ReadlineError, DefaultEditor, Result};
use yansi::Paint;

pub enum MetaCommandResult {
    Success,
    UnrecognizedCommand,
}

pub enum StatementCommandResult {
    Success(Statement),
    Unrecognized,
}

pub enum Statement {
    Insert,
    Select,
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Insert => write!(f, "insert"),
            Self::Select => write!(f, "select"),
        }
    }
}

const EXIT_SUCCESS: i32 = 0;

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline(&format!("{}", Paint::blue("lildb > ").bold()));
        match readline {
            Ok(line) => {
                if line.starts_with(".") {
                    match do_meta_command(&line) {
                        MetaCommandResult::Success => continue,
                        MetaCommandResult::UnrecognizedCommand => {
                            println!(
                                "Unrecognized command: {}",
                                Paint::red(format!("\'{}\'", line))
                            );
                            continue;
                        }
                    }
                }

                if line.is_empty() {
                    continue;
                }

                match prepare_statement(&line) {
                    StatementCommandResult::Success(statement) => {
                        println!(
                            "The command related to {} would be executed here.",
                            statement
                        );
                        println!("Executed.");
                        continue;
                    }
                    StatementCommandResult::Unrecognized => {
                        println!(
                            "Unrecognized keyword at start of {}",
                            Paint::red(format!("\'{}\'", line))
                        );
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

fn prepare_statement(line: &str) -> StatementCommandResult {
    if line.contains("insert") {
        return StatementCommandResult::Success(Statement::Insert);
    }

    if line.contains("select") {
        return StatementCommandResult::Success(Statement::Select);
    }

    StatementCommandResult::Unrecognized
}

fn do_meta_command(line: &str) -> MetaCommandResult {
    if line == ".exit" {
        println!("{}", Paint::green("Exiting...").bold());
        std::process::exit(EXIT_SUCCESS);
    }

    MetaCommandResult::UnrecognizedCommand
}
