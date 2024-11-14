mod scanner;
mod environment;

use environment::Environment;
use scanner::{Scanner, Token, TokenType, LiteralValue}; // Added `Token` to imports
use std::env;
use std::fs;
use std::io::{self, BufRead, Write, Error};
use std::process::exit;

fn run(contents: &str, env: &mut Environment) -> Result<(), String> {
    let mut scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens()?;  // Scan and store tokens
    let mut iter = tokens.into_iter().peekable();  // Use `into_iter()` to take ownership of tokens

    while let Some(token) = iter.next() {
        match token.token_type {
            TokenType::Var => {
                // Expect an identifier (variable name)
                if let Some(Token { token_type: TokenType::Identifier, lexeme, .. }) = iter.next() {
                    if let Some(Token { token_type: TokenType::Equal, .. }) = iter.next() {
                        let expr_value = evaluate_expression(&mut iter, env)?;  // Pass mutable reference to the iterator
                        env.define(lexeme.clone(), expr_value);
                    }
                }
            }
            TokenType::Identifier => {
                // Assignment or variable evaluation
                if let Some(Token { token_type: TokenType::Equal, .. }) = iter.peek() {
                    iter.next(); // Consume '=' token
                    let expr_value = evaluate_expression(&mut iter, env)?;  // Pass mutable reference to the iterator
                    env.assign(&token.lexeme, expr_value)?;
                } else {
                    if let Some(value) = env.get(&token.lexeme) {
                        println!("{} = {:?}", token.lexeme, value);
                    } else {
                        return Err(format!("Undefined variable '{}'", token.lexeme));
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}



fn evaluate_expression<I>(iter: &mut std::iter::Peekable<I>, env: &Environment) -> Result<LiteralValue, String>
where
    I: Iterator<Item = Token>,  // Now expecting owned tokens (not references)
{
    let mut result = match iter.next() {
        Some(Token {
            token_type: TokenType::Number,
            literal: Some(LiteralValue::IntValue(value)),
            ..
        }) => value,  // No need to dereference since it's already owned
        Some(Token {
            token_type: TokenType::Identifier,
            lexeme,
            ..
        }) => match env.get(&lexeme) {  // Use &lexeme to pass a reference to get()
            Some(LiteralValue::IntValue(value)) => *value,  // Dereference here to get `i64`
            _ => return Err(format!("Undefined variable '{}'", lexeme)),
        },
        _ => return Err("Expected a number or variable".to_string()),
    };

    while let Some(operator) = iter.peek() {
        match operator.token_type {
            TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash => {
                let operator_token = iter.next().unwrap();
                let next_value = match iter.next() {
                    Some(Token {
                        token_type: TokenType::Number,
                        literal: Some(LiteralValue::IntValue(value)),
                        ..
                    }) => value,
                    Some(Token {
                        token_type: TokenType::Identifier,
                        lexeme,
                        ..
                    }) => match env.get(&lexeme) {  // Use &lexeme to pass a reference to get()
                        Some(LiteralValue::IntValue(value)) => *value,  // Dereference here to get `i64`
                        _ => return Err(format!("Undefined variable '{}'", lexeme)),
                    },
                    _ => return Err("Expected a number or variable after operator".to_string()),
                };

                // Perform the operation
                result = match operator_token.token_type {
                    TokenType::Plus => result + next_value,
                    TokenType::Minus => result - next_value,
                    TokenType::Star => result * next_value,
                    TokenType::Slash => {
                        if next_value == 0 {
                            return Err("Division by zero".to_string());
                        }
                        result / next_value
                    }
                    _ => unreachable!(),
                };
            }
            _ => break,
        }
    }

    Ok(LiteralValue::IntValue(result))
}





fn run_file(path: &str, env: &mut Environment) -> Result<(), Error> {
    let contents = fs::read_to_string(path)?;
    match run(&contents, env) {
        Ok(()) => Ok(()),
        Err(msg) => Err(Error::new(std::io::ErrorKind::Other, msg)),
    }
}

fn run_prompt(env: &mut Environment) -> Result<(), String> {
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        match handle.read_line(&mut buffer) {
            Ok(n) => {
                if n <= 2 {
                    return Ok(());
                }
            }
            Err(_) => return Err("Couldn't read line".to_string()),
        }
        match run(&buffer, env) {
            Ok(_) => (),
            Err(msg) => println!("{}", msg),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut environment = Environment::new();

    if args.len() > 2 {
        println!("Usage: jlox [script]");
        exit(64);
    } else if args.len() == 2 {
        match run_file(&args[1], &mut environment) {
            Ok(_) => exit(0),
            Err(msg) => {
                eprintln!("Error:\n{}", msg);
                exit(1);
            }
        }
    } else {
        match run_prompt(&mut environment) {
            Ok(_) => (),
            Err(msg) => {
                eprintln!("Error:\n{}", msg);
                exit(1);
            }
        }
    }
}
