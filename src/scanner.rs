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
pub struct Token {
  // Define fields for the Token struct
}
