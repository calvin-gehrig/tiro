use super::token::Token;
use logos::Lexer;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexingError {
    InvalidToken(char),
    RegexStringError(String),
    #[default]
    Other
}

impl LexingError {
    pub fn from_lexer (lex: &mut Lexer<Token>) -> Self {
        Self::InvalidToken(lex.slice().chars().next().unwrap())
    }
}
