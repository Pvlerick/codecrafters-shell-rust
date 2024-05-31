#[allow(unused_imports)]
use std::ops::Deref;
use std::{
    env,
    error::Error,
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::{self, Command, Stdio},
    str::FromStr,
};

// Must be sorted alphabetically
static BUILTINS: &[(&str, fn(&[&str]) -> Result<(), Box<dyn Error>>)] = &[
    ("bye", exit),
    ("cd", cd),
    ("echo", echo),
    ("exit", exit),
    ("pwd", pwd),
    ("type", r#type),
];

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("$ ");
        io::stdout().flush()?;

        input.clear();
        stdin.read_line(&mut input)?;

        let input = input.split_whitespace().collect::<Vec<_>>();
        let command = input[0];
        let args = &input[1..];

        if let Ok(handler) = BUILTINS
            .binary_search_by(|(k, _)| k.cmp(&command))
            .map(|i| BUILTINS[i].1)
        {
            match handler(args) {
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                }
                _ => {}
            }
        } else {
            let path = Path::new(command);
            if path.exists() {
                exec(path, args);
            } else {
                match search_command_in_path(command) {
                    Some(path) => exec(&path, args),
                    _ => eprintln!("{}: command not found", command),
                }
            }
        }
    }
}

fn exit(_: &[&str]) -> Result<(), Box<dyn Error>> {
    process::exit(0);
}

fn echo(args: &[&str]) -> Result<(), Box<dyn Error>> {
    println!("{}", args.join(" "));

    Ok(())
}

fn r#type(args: &[&str]) -> Result<(), Box<dyn Error>> {
    let arg0 = args[0];

    match buildtin(arg0) {
        Some(_) => println!("{} is a shell builtin", arg0),
        _ => match search_command_in_path(arg0) {
            Some(path) => println!("{} is {}", arg0, path.display()),
            _ => println!("{} not found", arg0),
        },
    }

    Ok(())
}

fn pwd(_: &[&str]) -> Result<(), Box<dyn Error>> {
    println!("{}", env::current_dir()?.display());

    Ok(())
}

fn cd(args: &[&str]) -> Result<(), Box<dyn Error>> {
    let res = PathBuf::from_str(args[0]);
    match res {
        Ok(path) if path.exists() => env::set_current_dir(path)?,
        _ => return Err(format!("cd: {}: no such file or directory", args[0]).into()),
    };

    Ok(())
}

fn buildtin(command: &str) -> Option<fn(&[&str]) -> Result<(), Box<dyn Error>>> {
    BUILTINS
        .binary_search_by(|(k, _)| k.cmp(&command))
        .map(|i| BUILTINS[i].1)
        .ok()
}

fn search_command_in_path(command: &str) -> Option<PathBuf> {
    let paths = env::var("PATH")
        .map(|i| i.leak())
        .map_or(vec![], |i| i.split(":").collect());

    for path in paths {
        match fs::read_dir(path) {
            Ok(dir) => {
                for file in dir {
                    let file = file.unwrap();
                    if file.file_name() == command {
                        return Some(file.path());
                    }
                }
            }
            Err(_) => {} //TODO Handle non "Permission denied" error
        }
    }

    None
}

fn exec(path: &Path, args: &[&str]) {
    let command = Command::new(path)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let _ = command.wait_with_output();
}
