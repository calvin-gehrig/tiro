use lalrpop_util::lalrpop_mod;
use crate::lexer::Lexer;

use crate::common::Statement;

lalrpop_mod!(parser, "/parser/grammar.rs");
use parser::ProgramParser;

#[cfg(test)]
mod tests;

pub fn parse(lexer: Lexer) -> Vec<Statement> {
    ProgramParser::new().parse(lexer).expect("Parsing error")
}
