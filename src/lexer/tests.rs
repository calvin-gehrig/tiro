use crate::lexer::{
    token::Token,
    error::LexingError
};
use logos::Logos;

pub fn tokenize(code: &str) -> Result<Vec<Token>, LexingError> {
    Token::lexer(code).collect::<Result<_, _>>()
}

#[test]
fn echo() {
    assert_eq!(tokenize("echo"), Ok(vec![Token::Echo]));
}

#[test]
fn catena() {
    assert_eq!(tokenize("\"a string\""), Ok(vec![Token::Catena("a string".to_string() )]));
}

#[test]
fn ignore() {
    assert_eq!(tokenize(" \t\n"), Ok(vec![]));
}

#[test]
fn hello_world() {
    assert_eq!(tokenize("echo \"Hello,\"\necho \"world!\""), Ok(vec![
        Token::Echo,
        Token::Catena("Hello,".to_string()),
        Token::Echo,
        Token::Catena("world!".to_string())
    ]));
}

#[test]
fn invalid_token() {
    assert_eq!(tokenize("§"), Err(LexingError::InvalidToken('§')));
}
