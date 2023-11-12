use rustyline::{error::ReadlineError, DefaultEditor, Result};

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline("lildb > ");
        match readline {
            Ok(line) => {
                if line == ".exit" {
                    println!("Exiting...");
                    break;
                }
                println!("Unrecognized command: {}", line);
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
