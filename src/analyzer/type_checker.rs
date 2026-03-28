use crate::parser::ast::{
    Statement,
    Expression
};

mod error;
use error::{
    TypeCheckError,
    TypeError
};

#[cfg(test)]
mod tests;

pub fn type_check(mut ast: Vec<Statement>) -> Vec<Statement> {
    let mut analyzer = Analyzer::new();
    for statement in &mut ast {
        check_statement(statement, &mut analyzer);
    }
    ast
}

struct Analyzer {
    error_stack: Vec<TypeCheckError>,
    error_mode: bool
}

impl Analyzer {
    fn new () -> Self {
        Self {
            error_stack: Vec::new(),
            error_mode: false
        }
    }

    fn push_error(&mut self, error: TypeCheckError) {
        self.error_stack.push(error);
        if !self.error_mode {
            self.error_mode = true;
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum TiroType {
    StringType
}

fn check_statement(statement: &mut Statement, analyzer: &mut Analyzer) {
    match statement {
        Statement::Print {value} => check_print(value, analyzer),
    };
}

fn check_print(value: &mut Expression, analyzer: &mut Analyzer) {
    if let Some(tiro_type) = check_expression(value, analyzer) {
        if tiro_type != TiroType::StringType {
            analyzer.push_error(TypeCheckError::MismatchedTypeError(
                    TypeError::PrintValueError(TiroType::StringType)));
        }
    }
}

fn check_expression(expression: &mut Expression, analyzer: &mut Analyzer) -> Option<TiroType> {
    match expression {
        Expression::StringValue {..} => Some(TiroType::StringType),
    }
}
