use rustyline::{error::ReadlineError, DefaultEditor, Result};

pub enum MetaCommandResult {
    Success,
    UnrecognizedCommand,
}

const EXIT_SUCCESS: i32 = 0;

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline("lildb > ");
        match readline {
            Ok(line) => {
                if line.starts_with(".") {
                    match do_meta_command(&line) {
                        MetaCommandResult::Success => continue,
                        MetaCommandResult::UnrecognizedCommand => {
                            println!("Unrecognized command: {}", line);
                            continue;
                        }
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

fn do_meta_command(line: &str) -> MetaCommandResult {
    if line == ".exit" {
        println!("Exiting...");
        std::process::exit(EXIT_SUCCESS);
    }
    MetaCommandResult::UnrecognizedCommand
}
