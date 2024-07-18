pub struct Scanner {
  source: String,
}

impl Scanner {
  pub fn new(source: &str) -> Self {
      Self {
          source: source.to_string(),
      }
  }

  pub fn scan_tokens(&self) -> Result<Vec<Token>, String> {
      todo!()
  }
}

#[derive(Debug)]
pub enum TokenType {
  // Single character tokens
  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,
  Comma,
  Dot,
  Plus,
  Minus,
  Star,
  Slash,
  Start,
  Semicolon,

  // Multiple character tokens
  Bang,
  BangEqual,
  Equal,
  EqualEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,

  // Literals
  Identifier,
  String,
  Number,

  // Keywords
  And,
  Class,
  Warna,   // Else
  Agar,    // If
  Nope,    // Nil
  Or,
  For,
  Chap,    // Print
  Wapas,   // Return
  Supper,
  Sach,    // True
  Jhoot,   // False
  This,
  Var,
  Jabtak,  // While
  Eof,
}

#[derive(Debug)]
pub enum LiteralVal {
  IntValue(i64),
  FValue(f64),
  StringValue(String),
  IdentifierValue(String),
}

#[derive(Debug)]
pub struct Token {
  token_type: TokenType,
  lexeme: String,
  literal: Option<LiteralVal>,
  line_number: i64,
}

impl Token {
  pub fn new(
      token_type: TokenType,
      lexeme: String,
      literal: Option<LiteralVal>,
      line_number: i64,
  ) -> Self {
      Self {
          token_type,
          lexeme,
          literal,
          line_number,
      }
  }
}
