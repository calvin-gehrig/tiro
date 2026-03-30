use crate::common::{
    Statement,
    Expression,
    Parameter,
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
fn function_declaration() {
    assert_eq!(parse_test("fvnctio greet() echo \"Hello\" reddi"), vec![
        Statement::FunctionDeclaration {
            identifier: Symbol::Name("greet".to_string()),
            param_list: vec![],
            return_type: None,
            block: Box::new(vec![
                Statement::Print {
                    value: Expression::StringValue {value: "Hello".to_string()}
                },
                Statement::ReturnStatement {
                    return_value: None
                }
            ])
        }
    ]);
}

#[test]
fn str_id_declaration() {
    assert_eq!(parse_test("fvnctio str_id(a:cat, b:cat)=>cat reddi a"), vec![
        Statement::FunctionDeclaration {
            identifier: Symbol::Name("str_id".to_string()),
            param_list: vec![
                Parameter {
                    identifier: Symbol::Name("a".to_string()),
                    param_type: Symbol::Name("cat".to_string())
                },
                Parameter {
                    identifier: Symbol::Name("b".to_string()),
                    param_type: Symbol::Name("cat".to_string())
                }
            ],
            return_type: Some(Symbol::Name("cat".to_string())),
            block: Box::new(vec![
                Statement::ReturnStatement {
                    return_value: Some(Expression::Variable {
                                          identifier: Symbol::Name("a".to_string())
                    })
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
                identifier: Symbol::Name("a".to_string()),
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
