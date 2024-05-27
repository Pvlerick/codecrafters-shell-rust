#[allow(unused_imports)]
use std::ops::Deref;
use std::{
    io::{self, Write},
    process,
};

static BUILTINS: &[(&str, fn(&[&str]))] = &[("echo", echo), ("exit", exit), ("type", r#type)];

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

        if let Ok(handler) = BUILTINS
            .binary_search_by(|(k, _)| k.cmp(&command))
            .map(|i| BUILTINS[i].1)
        {
            handler(args);
        } else {
            eprintln!("{}: command not found", command);
        }
    }
}

fn exit(_: &[&str]) {
    process::exit(0);
}

fn echo(args: &[&str]) {
    println!("{}", args.join(" "));
}

fn r#type(args: &[&str]) {
    let arg0 = args[0];

    match buildtin(arg0) {
        Some(_) => println!("{} is a shell builtin", arg0),
        _ => println!("{} not found", arg0),
    }
}

fn buildtin(command: &str) -> Option<fn(&[&str])> {
    BUILTINS
        .binary_search_by(|(k, _)| k.cmp(&command))
        .map(|i| BUILTINS[i].1)
        .ok()
}
