extern crate your_project; // Replace with the actual name of your project

use your_project::scanner::{Scanner, TokenType};

#[test]
fn handle_one_char() {
    let source = "(())";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().expect("Failed to scan tokens");

    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0].token_type, TokenType::LeftParen);
    assert_eq!(tokens[1].token_type, TokenType::LeftParen);
    assert_eq!(tokens[2].token_type, TokenType::RightParen);
    assert_eq!(tokens[3].token_type, TokenType::RightParen);
    assert_eq!(tokens[4].token_type, TokenType::Eof);
}