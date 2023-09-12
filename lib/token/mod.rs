#[derive(Debug, PartialEq)]
pub enum Token {
  Illegal,
  Eof,

  // Identifiers + literals
  Ident(String),
  Int(i64),

  // Operators
  Assign,
  Plus,

  // Delimiters
  Comma,
  Semicolon,

  // Brackets
  LParen,
  RParen,
  LBrace,
  RBrace,

  // Keywords
  Function,
  Let,
}

impl Token {
  pub fn lookup_ident(ident: &str) -> Token {
    match ident {
      "fn" => Token::Function,
      "let" => Token::Let,
      _ => Token::Ident(ident.to_string()),
    }
  }
}