use std::io::{self, Write};
use monkeylang::{token::Token, lexer::Lexer};

fn main() {
  loop {
    print!("> ");
    let _ = io::stdout().flush();
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
      Ok(_) => {},
      Err(_) => {
        println!("Error reading input");
        break;
      }
    }

    let mut lexer = Lexer::new(&input);
    loop {
      let token = lexer.next_token();
      println!("{:?}", token);
      if token == Token::Eof {
        break;
      }
    }
  }
}
