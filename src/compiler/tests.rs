use crate::common::{
    AnalyzedAst,
    Symtable,
    Statement, 
    Expression,
    Function,
    LocalVariable,
    ParamType,
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
