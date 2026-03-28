use logos::{Logos, SpannedIter};

pub mod token;
use token::Token;

pub mod error;
use error::LexingError;

#[cfg(test)]
mod tests;

pub type Spanned<Token, Location, Error> = Result<(Location, Token, Location), Error>;

pub struct Lexer<'input> {
  token_stream: SpannedIter<'input, Token>,
}

impl<'input> Lexer<'input> {
  pub fn new(input: &'input str) -> Self {
    Self { token_stream: Token::lexer(input).spanned() }
  }
}

impl<'input> Iterator for Lexer<'input> {
  type Item = Spanned<Token, usize, LexingError>;

  fn next(&mut self) -> Option<Self::Item> {
    self.token_stream
      .next()
      .map(|(token, span)| Ok((span.start, token?, span.end)))
  }
}
