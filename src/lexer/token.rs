use logos::{Logos, Lexer};

use std::fmt;

use super::error::LexingError;

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(error(LexingError, LexingError::from_lexer))]
#[logos(skip r"[ \t\n\r]+")]
pub enum Token {
    #[regex("\"[^\"]*\"", parse_catena)]
    Catena(String),
    #[token("echo")]
    Echo,
    #[token("sit")]
    Sit,
    #[token("fvnctio")]
    Functio,
    #[token("reddi")]
    Reddi,
    #[token("voc")]
    Voc,
    #[token("=>")]
    RightArrow,
    #[token("(")]
    LeftParan,
    #[token(")")]
    RightParan,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token(",")]
    Comma,
    #[token(":")]
    Column,
    #[regex("[a-zA-Z_][a-zA-Z_0-9]*", parse_id)]
    Identifier(String)
}

impl fmt::Display for Token {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

fn parse_catena(lex: &mut Lexer<Token>) -> Result<String, LexingError> {
    let slice = lex.slice();
    if slice.len() >= 2 {
        Ok(slice[1..slice.len() - 1].to_string())
    } else {
        Err(LexingError::RegexStringError(slice.to_string()))
    }
}

fn parse_id(lex: &mut Lexer<Token>) -> String {
    lex.slice().to_string()
}
