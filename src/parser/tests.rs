use crate::common::{
    Statement,
    Expression,
    Parameter
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
            identifier: "_a1".to_string(),
            variable_type: Some("cat".to_string())
        }
    ]);
}

#[test]
fn print_variable() {
    assert_eq!(parse_test("echo a"), vec![
        Statement::Print {
            value: Expression::Variable {
                identifier: "a".to_string()
            }
        }
    ]);
}

#[test]
fn function_declaration() {
    assert_eq!(parse_test("fvnctio greet() echo \"Hello\" reddi"), vec![
        Statement::FunctionDeclaration {
            identifier: "greet".to_string(),
            param_list: vec![],
            return_type: None,
            block: Box::new(vec![
                Statement::Print {
                    value: Expression::StringValue {value: "Hello".to_string()}
                },
                Statement::ReturnStatement {
                    return_value: None,
                    function: "greet".to_string()
                }
            ])
        }
    ]);
}

#[test]
fn str_id_declaration() {
    assert_eq!(parse_test("fvnctio str_id(a:cat, b:cat)=>cat reddi a"), vec![
        Statement::FunctionDeclaration {
            identifier: "str_id".to_string(),
            param_list: vec![
                Parameter {
                    identifier: "a".to_string(),
                    param_type: "cat".to_string()
                },
                Parameter {
                    identifier: "b".to_string(),
                    param_type: "cat".to_string()
                }
            ],
            return_type: Some("cat".to_string()),
            block: Box::new(vec![
                Statement::ReturnStatement {
                    return_value: Some(Expression::Variable {
                                          identifier: "a".to_string()
                    }),
                    function: "str_id".to_string()
                }
            ])
        }
    ]);
}

#[test]
fn function_call() {
    assert_eq!(parse_test("voc a(\"a\")"), vec![
        Statement::Call {
            expression: Expression::FunctionCall {
                identifier: "a".to_string(),
                argument_list: Box::new(vec![
                    Expression::StringValue {
                        value: "a".to_string()
                    }
                ])
            }
        }]);
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
