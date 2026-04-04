use crate::common::{
    ResolvedAst,
    Symtable,
    Statement,
    Expression,
    ParamType,
    Function,
    LocalVariable,
    Type
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
                    identifier: 0
                },
                Statement::Print {
                    value: Expression::LocalVar {id: 0, depth: 0}
                }
            ],
            symtable: Symtable {
                variable_table: vec![LocalVariable {
                    vartype: Some(Type::StringType),
                    identifier: "a".to_string(),
                    index: 0
                }],
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
                identifier: 0,
                block: Box::new(vec![
                    Statement::ResolvedReturn {
                        return_value: Some(Expression::LocalVar {
                            id: 1,
                            depth: 0
                        }),
                        function: 0
                    }
                ])
            },
            Statement::Call {
            expression: Expression::ResolvedFunctionCall {
                id: 0,
                argument_list: Box::new(vec![
                    Expression::StringValue {value:"pff".to_string()},
                    Expression::StringValue {value:"aah".to_string()}
                ])
            }
        }],
        symtable: Symtable {
            variable_table: vec![
                LocalVariable {
                    vartype: Some(Type::StringType),
                    identifier: "a".to_string(),
                    index: 0
                },
                LocalVariable {
                    vartype: Some(Type::StringType),
                    identifier: "b".to_string(),
                    index: 1
                }
            ],
            function_table: vec![Function {
                identifier: "select".to_string(),
                return_type: Some(Type::StringType),
                param_list: vec![
                    ParamType {
                        identifier: "a".to_string(),
                        param_type: Type::StringType
                    },
                    ParamType {
                        identifier: "b".to_string(),
                        param_type: Type::StringType
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
                identifier: 0,
                block: Box::new(vec![
                    Statement::ResolvedReturn {
                        return_value: Some(Expression::StringValue {
                            value: "error".to_string(),
                        }),
                        function: 0
                    }
                ])
            }],
        symtable: Symtable {
            variable_table: vec![],
            function_table: vec![Function {
                identifier: "null".to_string(),
                return_type: None,
                param_list: vec![]
            }]
        },
        error_mode: false
    }), Some(TypeCheckError::ReturnError(
            ReturnErr::ValueReturnedOnNull("null".to_string())
        )));
}

#[test]
fn null_returned_on_value() {
    assert_eq!(type_check(ResolvedAst {
        ast: vec![
            Statement::FunctionDefinition {
                identifier: 0,
                block: Box::new(vec![
                    Statement::ResolvedReturn {
                        return_value: None,
                        function: 0
                    }
                ])
            }],
        symtable: Symtable {
            variable_table: vec![],
            function_table: vec![Function {
                identifier: "str".to_string(),
                return_type: Some(Type::StringType),
                param_list: vec![]
            }]
        },
        error_mode: false
    }), Some(TypeCheckError::ReturnError(
            ReturnErr::NullReturnedOnValue("str".to_string())
        )));
}

#[test]
fn arity_error() {
    assert_eq!(type_check(ResolvedAst {
        ast: vec![
            Statement::FunctionDefinition {
                identifier: 0,
                block: Box::new(vec![
                    Statement::ResolvedReturn {
                        return_value: None,
                        function: 0
                    }
                ])
            },
            Statement::Call {
            expression: Expression::ResolvedFunctionCall {
                id: 0,
                argument_list: Box::new(vec![
                    Expression::StringValue {value:"pff".to_string()},
                    Expression::StringValue {value:"aah".to_string()}
                ])
            }
        }],
        symtable: Symtable {
            variable_table: vec![
                LocalVariable {
                    vartype: Some(Type::StringType),
                    identifier: "a".to_string(),
                    index: 0
                }
            ],
            function_table: vec![Function {
                identifier: "id".to_string(),
                return_type: None,
                param_list: vec![
                    ParamType {
                        identifier: "a".to_string(),
                        param_type: Type::StringType
                    }
                ]
            }]
        },
        error_mode: false
    }), Some(TypeCheckError::ArityError(
        "id".to_string(),
        1, 2
        )));
}
