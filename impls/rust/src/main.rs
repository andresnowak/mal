use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::io::{self, Write};

mod step0_repl;

fn main() {
    let mut rl = Editor::<()>::new();

    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline("user> ");

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("{}", step0_repl::rep(&line));
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {}
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }

        rl.save_history("history.txt").unwrap();
    }
}
