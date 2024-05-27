#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    if let Some(command) = input.split_whitespace().collect::<Vec<_>>().first() {
        eprintln!("{}: command not found", command);
    }
}
