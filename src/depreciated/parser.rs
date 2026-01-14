use crate::lexer::Token;

mod expression;
pub use expression::Expression;

mod statement;
pub use statement::Statement;
use statement::parse_block;

type IterToken = std::iter::Peekable<std::vec::IntoIter<Token>>;

pub fn parse(tokens: Vec<Token>) -> Vec<Statement> {
    let (ast, _) = parse_block(tokens.into_iter().peekable());
    ast
}
