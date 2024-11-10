mod scanner;
use crate::scanner::{Scanner, Token};

use std::env;
use std::fs;
use std::io::{self, BufRead, Write, Error};
use std::process::exit;

fn run(contents: &str) -> Result<(), String> {
    let mut scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens()?;
    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}

fn run_file(path: &str) -> Result<(), Error> {
    let contents = fs::read_to_string(path)?;
    match run(&contents) {
        Ok(()) => Ok(()),
        Err(msg) => Err(Error::new(std::io::ErrorKind::Other, msg)),
    }
}

fn run_prompt() -> Result<(), String> {
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");  // Ensure prompt is shown
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        match handle.read_line(&mut buffer) {
            Ok(n) => {
                if n <= 2 { // Exit on empty input or newline
                    return Ok(());
                }
            }
            Err(_) => return Err("Couldn't read line".to_string()),
        }
        match run(&buffer) {
            Ok(_) => (),
            Err(msg) => println!("{}", msg),
        }
    }
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
}
