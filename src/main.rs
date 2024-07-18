use std::env;
use std::fs;
use std::io::Error;
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

fn run_prompt() {
    // Implementation for running in interactive mode (prompt)
    println!("Running in prompt mode...");
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
        run_prompt();
    }

    dbg!(args);
}
