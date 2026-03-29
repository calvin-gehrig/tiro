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
fn sit() {
    assert_eq!(tokenize("sit"), Ok(vec![Token::Sit]));
}

#[test]
fn column() {
    assert_eq!(tokenize(":"), Ok(vec![Token::Column]));
}

#[test]
fn identifier() {
    assert_eq!(tokenize("_abc12"), Ok(vec![Token::Identifier("_abc12".to_string())]));
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
fn declaration() {
    assert_eq!(tokenize("sit _a1:cat \"a\""), Ok(vec![
            Token::Sit,
            Token::Identifier("_a1".to_string()),
            Token::Column,
            Token::Identifier("cat".to_string()),
            Token::Catena("a".to_string())
    ]));
}

#[test]
fn invalid_token() {
    assert_eq!(tokenize("§"), Err(LexingError::InvalidToken('§')));
}
