use crate::common::{
    ResolvedAst,
    Symtable,
    Statement,
    Expression,
    Parameter,
    ParamType,
    Function,
    LocalVariable,
    Type
};

use super::{
    Resolver,
    resolve,
    resolve_block,
    register_global,
    error::ReferenceError
};

fn resolve_error(ast: Vec<Statement>) -> Vec<ReferenceError> {
    let mut resolver = Resolver::new();
    register_global(&ast, &mut resolver);
    resolve_block(ast, &mut resolver);
    resolver.error_stack
}

#[test]
fn variable() {
    assert_eq!(resolve(vec![
        Statement::VariableDeclaration {
            value: Expression::StringValue {value:"a".to_string()},
            identifier: "a".to_string(),
            variable_type: Some("cat".to_string())
        },
        Statement::Print {
            value: Expression::Variable {
                identifier: "a".to_string()
            }
        }
    ]), ResolvedAst {
        ast: vec![
            Statement::VariableAssignment {
                value: Expression::StringValue {value:"a".to_string()},
                identifier: 0
            },
            Statement::Print {
                value: Expression::LocalVar {
                    id: 0,
                    depth: 0
                }
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
    });
}

#[test]
fn function() {
    assert_eq!(resolve(vec![
        Statement::FunctionDeclaration {
            identifier: "select".to_string(),
            return_type: Some("cat".to_string()),
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
            block: Box::new(vec![
                Statement::ReturnStatement {
                    return_value: Some(Expression::Variable {
                        identifier: "b".to_string(),
                    }),
                    function: "select".to_string()
                }])
        },
        Statement::Call {
            expression: Expression::FunctionCall {
                identifier: "select".to_string(),
                argument_list: Box::new(vec![
                    Expression::StringValue {value:"pff".to_string()},
                    Expression::StringValue {value:"aah".to_string()}
                ])
            }
        }
    ]), ResolvedAst {
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
    });
}

#[test]
fn recursive_function() {
    assert_eq!(resolve(vec![
        Statement::FunctionDeclaration {
            identifier: "recur".to_string(),
            return_type: None,
            param_list: vec![],
            block: Box::new(vec![
                Statement::ReturnStatement {
                    return_value: Some(Expression::FunctionCall {
                        identifier: "recur".to_string(),
                        argument_list: Box::new(vec![])
                    }),
                    function: "recur".to_string()
                }])
        },
        Statement::Call {
            expression: Expression::FunctionCall {
                identifier: "recur".to_string(),
                argument_list: Box::new(vec![])
            }
        }
    ]), ResolvedAst {
        ast: vec![
            Statement::FunctionDefinition {
                identifier: 0,
                block: Box::new(vec![
                    Statement::ResolvedReturn {
                        return_value: Some(Expression::ResolvedFunctionCall {
                            id: 0,
                            argument_list: Box::new(vec![])
                        }),
                        function: 0
                    }
                ])
            },
            Statement::Call {
            expression: Expression::ResolvedFunctionCall {
                id: 0,
                argument_list: Box::new(vec![])
            }
        }],
        symtable: Symtable {
            variable_table: vec![],
            function_table: vec![Function {
                identifier: "recur".to_string(),
                return_type: None,
                param_list: vec![]
            }]
        },
        error_mode: false
    });
}


#[test]
fn variable_error() {
    assert_eq!(resolve_error(vec![
            Statement::Print {
                value: Expression::Variable {
                    identifier:"a".to_string()
                }
            }
    ]), vec![ReferenceError::UndefinedVariableName("a".to_string())]);
}

#[test]
fn symbol_as_variable_error() {
    assert_eq!(resolve_error(vec![
        Statement::FunctionDeclaration {
            identifier: "select".to_string(),
            return_type: Some("cat".to_string()),
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
            block: Box::new(vec![
                Statement::ReturnStatement {
                    return_value: Some(Expression::Variable {
                        identifier: "b".to_string()
                    }),
                    function: "select".to_string()
                }])
        },
        Statement::Call {
            expression: Expression::Variable {
                identifier: "select".to_string()
            }
        }
    ]), vec![ReferenceError::InvalidSymbolUseAsVariable("select".to_string())]);
}

#[test]
fn function_error() {
    assert_eq!(resolve_error(vec![Statement::Call {
            expression: Expression::FunctionCall {
                identifier: "select".to_string(),
                argument_list: Box::new(vec![
                    Expression::StringValue {value:"pff".to_string()},
                    Expression::StringValue {value:"aah".to_string()}
                ])
            }
        }]), vec![ReferenceError::UndefinedFunctionName("select".to_string())]);
}

#[test]
fn symbol_as_function_error() {
    assert_eq!(resolve_error(vec![
            Statement::VariableDeclaration {
                value: Expression::StringValue {value:"a".to_string()},
                identifier: "a".to_string(),
                variable_type: Some("cat".to_string())
            },
            Statement::Call {
                expression: Expression::FunctionCall {
                    identifier: "a".to_string(),
                    argument_list: Box::new(vec![])
                }
            }
    ]), vec![ReferenceError::InvalidSymbolUseAsFunction("a".to_string())]);
}

#[test]
fn type_error() {
    assert_eq!(resolve_error(vec![
            Statement::VariableDeclaration {
                value: Expression::StringValue {value:"a".to_string()},
                identifier: "a".to_string(),
                variable_type: Some("ouh".to_string())
            }
    ]), vec![ReferenceError::UndefinedTypeName("ouh".to_string())]);
}
