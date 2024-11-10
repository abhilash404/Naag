use crate::scanner::*;

#[test]
fn handle_one_char() {
    let source = "( ) { }";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().expect("Failed to scan tokens");

    assert_eq!(tokens.len(), 5); // 4 characters + EOF token
    assert_eq!(tokens[0].token_type, TokenType::LeftParen);
    assert_eq!(tokens[1].token_type, TokenType::RightParen);
    assert_eq!(tokens[2].token_type, TokenType::LeftBrace);
    assert_eq!(tokens[3].token_type, TokenType::RightBrace);
    assert_eq!(tokens[4].token_type, TokenType::Eof);
}

#[test]
fn handle_twochar_tokens() {
    let source = "! != == >=";
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().expect("Failed to scan tokens");

    assert_eq!(tokens.len(), 5); // 4 tokens + EOF
    assert_eq!(tokens[0].token_type, TokenType::Bang);
    assert_eq!(tokens[1].token_type, TokenType::BangEqual);
    assert_eq!(tokens[2].token_type, TokenType::EqualEqual);
    assert_eq!(tokens[3].token_type, TokenType::GreaterEqual);
    assert_eq!(tokens[4].token_type, TokenType::Eof);
}
