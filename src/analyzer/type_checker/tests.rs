use crate::common::{
    ResolvedAst,
    Symtable,
    Statement,
    Expression,
    ParamType,
    Function,
    Symbol,
    TiroType
};

use super::{
    TypeChecker,
    check_statement,
    error::{
        TypeCheckError,
        ReturnErr,
        TypeError
    }
};

fn type_check(mut resolved_ast: ResolvedAst) -> Option<TypeCheckError> {
    let mut type_checker = TypeChecker::new(resolved_ast.symtable, resolved_ast.error_mode);
    for statement in &mut resolved_ast.ast {
        check_statement(statement, &mut type_checker);
    }
    match type_checker.error_stack.len() {
        0 => None,
        1 => Some(type_checker.error_stack[0].clone()),
        _ => panic!("Unexpected number of error")
    }
}

#[test]
fn print () {
    assert_eq!(type_check(ResolvedAst {
            ast: vec![
                Statement::Print {
                    value: Expression::StringValue {value: "a".to_string()}
                }
            ],
            symtable: Symtable {
                variable_table: vec![],
                function_table: vec![]
            },
            error_mode: false
    }), None);
}

#[test]
fn variable_assignment () {
    assert_eq!(type_check(ResolvedAst {
            ast: vec![
                Statement::VariableAssignment {
                    value: Expression::StringValue {value: "a".to_string()},
                    identifier: Symbol::Id(0)
                },
                Statement::Print {
                    value: Expression::Variable {identifier: Symbol::Id(0)}
                }
            ],
            symtable: Symtable {
                variable_table: vec![Some(TiroType::StringType)],
                function_table: vec![]
            },
            error_mode: false
    }), None);
}

#[test]
fn function_declaration() {
    assert_eq!(type_check(ResolvedAst {
        ast: vec![
            Statement::FunctionDefinition {
                identifier: Symbol::Id(0),
                block: Box::new(vec![
                    Statement::ReturnStatement {
                        return_value: Some(Expression::Variable {
                            identifier: Symbol::Id(1),
                        }),
                        function: Symbol::Id(0)
                    }
                ])
            },
            Statement::Call {
            expression: Expression::FunctionCall {
                identifier: Symbol::Id(0),
                argument_list: Box::new(vec![
                    Expression::StringValue {value:"pff".to_string()},
                    Expression::StringValue {value:"aah".to_string()}
                ])
            }
        }],
        symtable: Symtable {
            variable_table: vec![
                Some(TiroType::StringType),
                Some(TiroType::StringType)
            ],
            function_table: vec![Function {
                return_type: Some(TiroType::StringType),
                param_list: vec![
                    ParamType {
                        identifier: Symbol::Id(0),
                        param_type: TiroType::StringType
                    },
                    ParamType {
                        identifier: Symbol::Id(1),
                        param_type: TiroType::StringType
                    }
                ]
            }]
        },
        error_mode: false
    }), None);
}

#[test]
fn value_returned_on_null() {
    assert_eq!(type_check(ResolvedAst {
        ast: vec![
            Statement::FunctionDefinition {
                identifier: Symbol::Id(0),
                block: Box::new(vec![
                    Statement::ReturnStatement {
                        return_value: Some(Expression::StringValue {
                            value: "error".to_string(),
                        }),
                        function: Symbol::Id(0)
                    }
                ])
            }],
        symtable: Symtable {
            variable_table: vec![],
            function_table: vec![Function {
                return_type: None,
                param_list: vec![]
            }]
        },
        error_mode: false
    }), Some(TypeCheckError::ReturnError(
            ReturnErr::ValueReturnedOnNull)));
}

#[test]
fn null_returned_on_value() {
    assert_eq!(type_check(ResolvedAst {
        ast: vec![
            Statement::FunctionDefinition {
                identifier: Symbol::Id(0),
                block: Box::new(vec![
                    Statement::ReturnStatement {
                        return_value: None,
                        function: Symbol::Id(0)
                    }
                ])
            }],
        symtable: Symtable {
            variable_table: vec![],
            function_table: vec![Function {
                return_type: Some(TiroType::StringType),
                param_list: vec![]
            }]
        },
        error_mode: false
    }), Some(TypeCheckError::ReturnError(
            ReturnErr::NullReturnedOnValue)));
}

#[test]
fn arity_error() {
    assert_eq!(type_check(ResolvedAst {
        ast: vec![
            Statement::FunctionDefinition {
                identifier: Symbol::Id(0),
                block: Box::new(vec![
                    Statement::ReturnStatement {
                        return_value: None,
                        function: Symbol::Id(0)
                    }
                ])
            },
            Statement::Call {
            expression: Expression::FunctionCall {
                identifier: Symbol::Id(0),
                argument_list: Box::new(vec![
                    Expression::StringValue {value:"pff".to_string()},
                    Expression::StringValue {value:"aah".to_string()}
                ])
            }
        }],
        symtable: Symtable {
            variable_table: vec![
                Some(TiroType::StringType),
            ],
            function_table: vec![Function {
                return_type: None,
                param_list: vec![
                    ParamType {
                        identifier: Symbol::Id(0),
                        param_type: TiroType::StringType
                    }
                ]
            }]
        },
        error_mode: false
    }), Some(TypeCheckError::ArityError(1, 2)));
}
