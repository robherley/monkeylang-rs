use crate::token::Token;

const NUL: char = '\0';

pub struct Lexer<'a> {
  iter: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer <'a> {
  pub fn new(input: &'a String) -> Self {
    Lexer {
      iter: input.chars().peekable(),
    }
  }

  fn read_char(&mut self) -> char {
    self.iter.next().unwrap_or(NUL).clone()
  }

  fn read_ident(&mut self, ch: &char) -> Token {
    let mut identifer = String::new();
    identifer.push(*ch);
    while match self.iter.peek() {
      Some(ch) => is_ident_char(*ch),
      None => false,
    } {
      identifer.push(self.read_char());
    }

    Token::lookup_ident(identifer.as_str())
  }

  fn read_int(&mut self, ch: &char) -> Token {
    let mut int = String::new();
    int.push(*ch);
    while match self.iter.peek() {
      Some(ch) => ch.is_digit(10),
      None => false,
    } {
      int.push(self.read_char());
    }

    match int.parse::<i64>() {
      Ok(int) => Token::Int(int),
      Err(_) => Token::Illegal,
    }
  }

  pub fn next_token(&mut self) -> Token {
    match self.read_char() {
      '=' => Token::Assign,
      ';' => Token::Semicolon,
      '(' => Token::LParen,
      ')' => Token::RParen,
      ',' => Token::Comma,
      '+' => Token::Plus,
      '{' => Token::LBrace,
      '}' => Token::RBrace,
      ch if is_ident_char(ch) => self.read_ident(&ch),
      ch if ch.is_digit(10) => self.read_int(&ch),
      ch if ch.is_whitespace() => self.next_token(),
      NUL => Token::Eof,
      _ => Token::Illegal,
    }
  }
}

fn is_ident_char(ch: char) -> bool {
  match ch {
    'a'..='z' | 'A'..='Z' | '_' => true,
    _ => false,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_next_token() {
    let input = "let five = 5;
let ten = 10;
let add = fn(x, y) {
  x + y;
};
let result = add(five, ten);
".to_string();

    let mut lexer = Lexer::new(&input);

    let expected = vec![
      Token::Let,
      Token::Ident("five".to_string()),
      Token::Assign,
      Token::Int(5),
      Token::Semicolon,
      Token::Let,
      Token::Ident("ten".to_string()),
      Token::Assign,
      Token::Int(10),
      Token::Semicolon,
      Token::Let,
      Token::Ident("add".to_string()),
      Token::Assign,
      Token::Function,
      Token::LParen,
      Token::Ident("x".to_string()),
      Token::Comma,
      Token::Ident("y".to_string()),
      Token::RParen,
      Token::LBrace,
      Token::Ident("x".to_string()),
      Token::Plus,
      Token::Ident("y".to_string()),
      Token::Semicolon,
      Token::RBrace,
      Token::Semicolon,
      Token::Let,
      Token::Ident("result".to_string()),
      Token::Assign,
      Token::Ident("add".to_string()),
      Token::LParen,
      Token::Ident("five".to_string()),
      Token::Comma,
      Token::Ident("ten".to_string()),
      Token::RParen,
      Token::Semicolon,
      Token::Eof,
    ];

    for expected_token in expected {
      let token = lexer.next_token();
      assert_eq!(token, expected_token);
    }
  }
}