use std::io::{self, Write};
#[allow(unused_imports)]
use std::{ops::Deref, process::exit};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        input.clear();
        stdin.read_line(&mut input).unwrap();

        let input = input.split_whitespace().collect::<Vec<_>>();
        let command = input[0];
        let args = &input[1..];

        match command {
            "exit" => exit(0),
            "echo" => println!("{}", args.join(" ")),
            "type" => {
                let arg0 = args[0];
                match arg0 {
                    "exit" | "echo" | "type" => println!("{} is a shell builtin", arg0),
                    _ => println!("{} not found", arg0),
                }
            }
            _ => eprintln!("{}: command not found", command),
        }
    }
}
