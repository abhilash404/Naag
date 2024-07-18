use std::env;
use std::fs;
use std::io::{self, BufRead, Error};
use std::process::exit;

fn run(_contents: &str) -> Result<(), String> {
    Err("Not implemented".to_string())
}

fn run_file(path: &str) -> Result<(), Error> {
    let contents = fs::read_to_string(path)?;
    match run(&contents) {
        Ok(()) => Ok(()),
        Err(msg) => Err(Error::new(std::io::ErrorKind::Other, msg)),
    }
}

fn run_prompt() -> Result<(), String> {
    print!("> ");
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    match handle.read_line(&mut buffer) {
        Ok(_) => (),
        Err(_) => return Err("Couldn't read line".to_string()),
    }
    println!("You wrote: {}", buffer);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 2 {
        println!("Usage: jlox [script]");
        exit(64);
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => exit(0),
            Err(msg) => {
                eprintln!("Error:\n{}", msg);
                exit(1);
            }
        }
    } else {
        match run_prompt() {
            Ok(_) => (),
            Err(msg) => {
                eprintln!("Error:\n{}", msg);
                exit(1);
            }
        }
    }

    dbg!(args);
}
