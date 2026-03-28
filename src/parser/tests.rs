use super::ast::{
    Statement,
    Expression
};

use crate::lexer::Lexer;

use super::parse;

fn parse_test(src: &str) -> Vec<Statement> {
    let lexer = Lexer::new(&src);
    parse(lexer)
}

#[test]
fn print() {
    assert_eq!(parse_test("echo \"a\""), vec![
        Statement::Print {
            value: Expression::StringValue {
                value: "a".to_string()
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
