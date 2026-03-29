use super::ast::{
    Statement,
    Expression,
    Symbol
};

use crate::lexer::Lexer;

use super::parse;

fn parse_test(src: &str) -> Vec<Statement> {
    let lexer = Lexer::new(&src);
    parse(lexer)
}

#[test]
fn print_string() {
    assert_eq!(parse_test("echo \"a\""), vec![
        Statement::Print {
            value: Expression::StringValue {
                value: "a".to_string()
            }
        }
    ]);
}

#[test]
fn variableDeclaration() {
    assert_eq!(parse_test("sit _a1:cat \"a\""), vec![
        Statement::VariableDeclaration {
            value: Expression::StringValue {
                value: "a".to_string()
            },
            identifier: Symbol::Name("_a1".to_string()),
            variable_type: Some(Symbol::Name("cat".to_string()))
        }
    ]);
}

#[test]
fn print_variable() {
    assert_eq!(parse_test("echo a"), vec![
        Statement::Print {
            value: Expression::Variable {
                identifier: Symbol::Name("a".to_string())
            }
        }
    ]);
}

#[test]
#[should_panic(expected = "Parsing error: UnrecognizedToken")]
fn parser_error() {
    parse_test("echo echo");
}

#[test]
#[should_panic(expected = "Parsing error: User { error: InvalidToken")]
fn lexer_error() {
    parse_test("§");
}
