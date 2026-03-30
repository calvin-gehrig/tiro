use crate::common::{
    ResolvedAst,
    Symtable,
    Statement,
    Expression,
    Symbol,
    Parameter,
    ParamType,
    Function,
    TiroType
};

use super::{
    Resolver,
    resolve,
    resolve_block,
    error::ReferenceError
};

fn resolve_error(ast: Vec<Statement>) -> Vec<ReferenceError> {
    let mut resolver = Resolver::new();
    resolve_block(ast, &mut resolver);
    resolver.error_stack
}

#[test]
fn variable() {
    assert_eq!(resolve(vec![
        Statement::VariableDeclaration {
            value: Expression::StringValue {value:"a".to_string()},
            identifier: Symbol::Name("a".to_string()),
            variable_type: Some(Symbol::Name("cat".to_string()))
        },
        Statement::Print {
            value: Expression::Variable {
                identifier:Symbol::Name("a".to_string())
            }
        }
    ]), ResolvedAst {
        ast: vec![
            Statement::VariableAssignment {
                value: Expression::StringValue {value:"a".to_string()},
                identifier: Symbol::Id(0)
            },
            Statement::Print {
                value: Expression::Variable {
                    identifier:Symbol::Id(0)
                }
            }
        ],
        symtable: Symtable {
            variable_table: vec![Some(TiroType::StringType)],
            function_table: vec![]
        },
        error_mode: false
    });
}

#[test]
fn function() {
    assert_eq!(resolve(vec![
        Statement::FunctionDeclaration {
            identifier: Symbol::Name("select".to_string()),
            return_type: Some(Symbol::Name("cat".to_string())),
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
            block: Box::new(vec![
                Statement::ReturnStatement {
                    return_value: Some(Expression::Variable {
                        identifier: Symbol::Name("b".to_string()),
                    }),
                    function: Symbol::Name("select".to_string())
                }])
        },
        Statement::Call {
            expression: Expression::FunctionCall {
                identifier: Symbol::Name("select".to_string()),
                argument_list: Box::new(vec![
                    Expression::StringValue {value:"pff".to_string()},
                    Expression::StringValue {value:"aah".to_string()}
                ])
            }
        }
    ]), ResolvedAst {
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
    });
}

#[test]
fn recursive_function() {
    assert_eq!(resolve(vec![
        Statement::FunctionDeclaration {
            identifier: Symbol::Name("rec".to_string()),
            return_type: None,
            param_list: vec![],
            block: Box::new(vec![
                Statement::ReturnStatement {
                    return_value: Some(Expression::FunctionCall {
                        identifier: Symbol::Name("rec".to_string()),
                        argument_list: Box::new(vec![])
                    }),
                    function: Symbol::Name("rec".to_string())
                }])
        },
        Statement::Call {
            expression: Expression::FunctionCall {
                identifier: Symbol::Name("rec".to_string()),
                argument_list: Box::new(vec![])
            }
        }
    ]), ResolvedAst {
        ast: vec![
            Statement::FunctionDefinition {
                identifier: Symbol::Id(0),
                block: Box::new(vec![
                    Statement::ReturnStatement {
                        return_value: Some(Expression::FunctionCall {
                            identifier: Symbol::Id(0),
                            argument_list: Box::new(vec![])
                        }),
                        function: Symbol::Id(0)
                    }
                ])
            },
            Statement::Call {
            expression: Expression::FunctionCall {
                identifier: Symbol::Id(0),
                argument_list: Box::new(vec![])
            }
        }],
        symtable: Symtable {
            variable_table: vec![],
            function_table: vec![Function {
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
                    identifier:Symbol::Name("a".to_string())
                }
            }
    ]), vec![ReferenceError::UndefinedVariableName("a".to_string())]);
}

#[test]
fn symbol_as_variable_error() {
    assert_eq!(resolve_error(vec![
        Statement::FunctionDeclaration {
            identifier: Symbol::Name("select".to_string()),
            return_type: Some(Symbol::Name("cat".to_string())),
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
            block: Box::new(vec![
                Statement::ReturnStatement {
                    return_value: Some(Expression::Variable {
                        identifier: Symbol::Name("b".to_string())
                    }),
                    function: Symbol::Name("select".to_string())
                }])
        },
        Statement::Call {
            expression: Expression::Variable {
                identifier: Symbol::Name("select".to_string()),
            }
        }
    ]), vec![ReferenceError::InvalidSymbolUseAsVariable("select".to_string())]);
}

#[test]
fn function_error() {
    assert_eq!(resolve_error(vec![Statement::Call {
            expression: Expression::FunctionCall {
                identifier: Symbol::Name("select".to_string()),
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
                identifier: Symbol::Name("a".to_string()),
                variable_type: Some(Symbol::Name("cat".to_string()))
            },
            Statement::Call {
                expression: Expression::FunctionCall {
                    identifier: Symbol::Name("a".to_string()),
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
                identifier: Symbol::Name("a".to_string()),
                variable_type: Some(Symbol::Name("ouh".to_string()))
            }
    ]), vec![ReferenceError::UndefinedTypeName("ouh".to_string())]);
}
