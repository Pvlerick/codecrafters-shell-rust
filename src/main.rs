#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        input.clear();
        stdin.read_line(&mut input).unwrap();

        match input.as_str() {
            "exit 0\n" => exit(0),
            _ => {
                if let Some(command) = input.split_whitespace().collect::<Vec<_>>().first() {
                    eprintln!("{}: command not found", command);
                }
            }
        }
    }
}
