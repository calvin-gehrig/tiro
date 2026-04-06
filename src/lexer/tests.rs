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
fn functio() {
    assert_eq!(tokenize("fvnctio"), Ok(vec![Token::Functio]));
}

#[test]
fn right_arrow() {
    assert_eq!(tokenize("=>"), Ok(vec![Token::RightArrow]));
}

#[test]
fn paran() {
    assert_eq!(tokenize("()"), Ok(vec![Token::LeftParan, Token::RightParan]));
}

#[test]
fn brace() {
    assert_eq!(tokenize("{}"), Ok(vec![Token::LeftBrace, Token::RightBrace]));
}

#[test]
fn comma() {
    assert_eq!(tokenize(","), Ok(vec![Token::Comma]));
}

#[test]
fn voc() {
    assert_eq!(tokenize("voc"), Ok(vec![Token::Voc]));
}

#[test]
fn number() {
    assert_eq!(tokenize("12_000_004"), Ok(vec![Token::Number(12000004)]));
}

#[test]
fn operations() {
    assert_eq!(tokenize("+-*/^~"),
    Ok(vec![
        Token::Plus,
        Token::Minus,
        Token::Star,
        Token::Slash,
        Token::Caret,
        Token::Tile
    ]));
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
fn variable_declaration() {
    assert_eq!(tokenize("sit _a1:cat \"a\""), Ok(vec![
            Token::Sit,
            Token::Identifier("_a1".to_string()),
            Token::Column,
            Token::Identifier("cat".to_string()),
            Token::Catena("a".to_string())
    ]));
}

#[test]
fn function_declaration() {
    assert_eq!(tokenize("fvnctio greet()=>nil echo \"Hello\" reddi"), Ok(vec![
            Token::Functio,
            Token::Identifier("greet".to_string()),
            Token::LeftParan,
            Token::RightParan,
            Token::RightArrow,
            Token::Identifier("nil".to_string()),
            Token::Echo,
            Token::Catena("Hello".to_string()),
            Token::Reddi
    ]));
}

#[test]
fn function_call() {
    assert_eq!(tokenize("voc greet()"), Ok(vec![
            Token::Voc,
            Token::Identifier("greet".to_string()),
            Token::LeftParan,
            Token::RightParan,
    ]));
}

#[test]
fn invalid_token() {
    assert_eq!(tokenize("§"), Err(LexingError::InvalidToken('§')));
}
