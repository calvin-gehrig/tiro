use crate::parser::ast::Statement;

mod type_checker;
use type_checker::type_check;

pub fn analyze(ast: Vec<Statement>) -> Vec<Statement> {
    type_check(ast)
}
