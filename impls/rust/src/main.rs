use std::io::{self, Write};
mod step0_repl;

fn main() {
    loop {
        let mut input = String::new();

        print!("user> ");
        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        print!("{}", step0_repl::rep(&input));
        let _ = io::stdout().flush();
    }
}
