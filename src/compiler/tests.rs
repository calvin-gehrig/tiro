use crate::common::{
    AnalyzedAst,
    Symtable,
    Statement, 
    Expression,
    Function,
    LocalVariable,
    ParamType,
    OperationType,
    Type
};

use super::{
    compile,
    CompiledProgram,
    Opcode,
    StackValue
};

#[test]
fn print() {
    assert_eq!(compile(AnalyzedAst {
            ast: vec![
            Statement::Print {
                value:
                    Expression::StringValue {
                        value: "a".to_string()
                    }
            }],
            symtable: Symtable {
                variable_table:vec![],
                function_table:vec![]
            }
        }), 
        CompiledProgram {
            main_program: vec![
                Opcode::Push(StackValue::StringIndex(0)),
                Opcode::Print
            ],
            string_pool: vec!["a".to_string()],
            function_pool: vec![]
        });
}

#[test]
fn variable_declaration() {
    assert_eq!(compile(AnalyzedAst {
            ast: vec![
                Statement::VariableAssignment {
                    value:
                        Expression::StringValue {
                            value: "a".to_string()
                        },
                    identifier: 0
                },
                Statement::Print {
                    value:
                        Expression::LocalVar {
                            id: 0,
                            depth: 0
                        }
                }
            ],
            symtable: Symtable {
                variable_table:vec![LocalVariable {
                    vartype: Some(Type::StringType),
                    identifier: "a".to_string(),
                    index: 0
                }],
                function_table:vec![]
            }
        }), CompiledProgram {
                main_program: vec![
                    Opcode::Push(StackValue::StringIndex(0)),
                    Opcode::LoadLocal(0, 0),
                    Opcode::Print
                ],
                string_pool: vec!["a".to_string()],
                function_pool: vec![]
        });
}

#[test]
fn function_declaration() {
    assert_eq!(compile(AnalyzedAst {
            ast: vec![
                Statement::FunctionDefinition {
                    block: Box::new(vec![
                        Statement::ResolvedReturn {
                            function: 0,
                            return_value: Some(Expression::LocalVar {
                                id: 0,
                                depth: 0
                            })
                        }
                    ]),
                    identifier: 0
                },
                Statement::Call {
                    expression:
                        Expression::ResolvedFunctionCall {
                            id: 0,
                            argument_list: Box::new(vec![
                                Expression::StringValue {
                                    value: "a".to_string()
                                }
                            ])
                        }
                }
            ],
            symtable: Symtable {
                variable_table:vec![LocalVariable {
                    vartype: Some(Type::StringType),
                    identifier: "a".to_string(),
                    index: 0
                }],
                function_table:vec![Function {
                    return_type: Some(Type::StringType),
                    param_list: vec![
                        ParamType {
                            identifier: "a".to_string(),
                            param_type: Type::StringType
                        }
                    ],
                    identifier: "id".to_string()
                }]
       }
        }), CompiledProgram {
                main_program: vec![
                    Opcode::Push(StackValue::StringIndex(0)),
                    Opcode::Call(0, 1),
                    Opcode::Pop
                ],
                string_pool: vec!["a".to_string()],
                function_pool: vec![
                    vec![
                        Opcode::LoadLocal(0,0),
                        Opcode::Return
                    ]
                ]
        });
}

#[test]
fn math_operation() {
    assert_eq!(compile(AnalyzedAst {
            ast: vec![
            Statement::Call {
                expression:
                    Expression::BinaryOperation {
                        op_type: OperationType::Add,
                        lhs: Box::new(Expression::BinaryOperation {
                            op_type: OperationType::Sub,
                            lhs: Box::new(Expression::BinaryOperation {
                                op_type: OperationType::Mul,
                                lhs: Box::new(Expression::Number {
                                    value: 2
                                }),
                                rhs: Box::new(Expression::Number {
                                    value: 3
                                })
                            }),
                            rhs: Box::new(Expression::Number {
                                value: 5
                            })
                        }),
                        rhs: Box::new(Expression::BinaryOperation {
                            op_type: OperationType::Div,
                            lhs: Box::new(Expression::BinaryOperation {
                                op_type: OperationType::Pow,
                                lhs: Box::new(Expression::Number {
                                    value: 2
                                }),
                                rhs: Box::new(Expression::Number {
                                    value: 4
                                })
                            }),
                            rhs: Box::new(Expression::Number {
                                value: 6
                            })
                        })
                    }
            }],
            symtable: Symtable {
                variable_table:vec![],
                function_table:vec![]
            }
        }), 
        CompiledProgram {
            main_program: vec![
                Opcode::Push(StackValue::Number(6)),
                Opcode::Push(StackValue::Number(4)),
                Opcode::Push(StackValue::Number(2)),
                Opcode::Pow,
                Opcode::Div,
                Opcode::Push(StackValue::Number(5)),
                Opcode::Push(StackValue::Number(3)),
                Opcode::Push(StackValue::Number(2)),
                Opcode::Mul,
                Opcode::Sub,
                Opcode::Add,
                Opcode::Pop
            ],
            string_pool: vec![],
            function_pool: vec![]
        });
}

#[test]
fn concat() {
    assert_eq!(compile(AnalyzedAst {
            ast: vec![
            Statement::Call {
                expression:
                    Expression::BinaryOperation {
                        op_type: OperationType::Cat,
                        lhs: Box::new(Expression::StringValue {
                            value: "a".to_string()
                        }),
                        rhs: Box::new(Expression::StringValue {
                            value: "b".to_string()
                        })
                    }
            }],
            symtable: Symtable {
                variable_table:vec![],
                function_table:vec![]
            }
        }), 
        CompiledProgram {
            main_program: vec![
                Opcode::Push(StackValue::StringIndex(0)),
                Opcode::Push(StackValue::StringIndex(1)),
                Opcode::Cat,
                Opcode::Pop
            ],
            string_pool: vec!["b".to_string(), "a".to_string()],
            function_pool: vec![]
        });
}
