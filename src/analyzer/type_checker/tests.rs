use crate::common::{
    ResolvedAst,
    Symtable,
    Statement,
    Expression,
    ParamType,
    Function,
    LocalVariable,
    Type,
    OperationType
};
use crate::{
    opt,
    stmt,
    expr,
    res_ast
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
    assert_eq!(type_check(
        res_ast!(
            ast [
                stmt!(print expr!(stri "a"))
            ])
    ), None);
}

#[test]
fn variable_assignment () {
    assert_eq!(type_check(
        res_ast!(
            ast [
                stmt!(assign 0 expr!(stri "a")),
                stmt!(print expr!(loc 0 0))
            ]
            var [ "a" 0 StringType ])
    ), None);
}

#[test]
fn function_declaration() {
    assert_eq!(type_check(
        res_ast!(
            ast [
                stmt!(def 0 {} expr!(loc 1 0)),
                stmt!(call expr!(rescall 0 (
                            expr!(stri "pff"),
                            expr!(stri "aah")
                )))
            ]
            var [ "a" 0 StringType, "b" 1 StringType]
            func [ "select" ("a" StringType, "b" StringType) StringType])
    ), None);
}

#[test]
fn math_operation() {
    assert_eq!(type_check(
        res_ast!(
            ast [
                stmt!(call expr!(bin Add
                        expr!(num 2),
                        expr!(num 3)
                ))
            ])
   ), None);
}

#[test]
fn string_operation() {
    assert_eq!(type_check(
        res_ast!(
            ast [
                stmt!(call expr!(bin Cat
                        expr!(stri "a"),
                        expr!(stri "b")
                ))
            ]
        )
    ), None);
}

#[test]
fn type_inference() {
    assert_eq!(type_check(
        res_ast!(
            ast [
                stmt!(assign 0 expr!(stri "a")),
                stmt!(print expr!(loc 0 0))
            ]
            var ["string" 0])
    ), None);
}

#[test]
fn value_returned_on_null() {
    assert_eq!(type_check(
        res_ast!(
            ast [
                stmt!(def 0 {} expr!(stri "error"))
            ]
            func [ "null" ()])
    ), Some(TypeCheckError::ReturnError(
            ReturnErr::ValueReturnedOnNull("null".to_string())
    )));
}

#[test]
fn null_returned_on_value() {
    assert_eq!(type_check(
        res_ast!(
            ast [
                stmt!(def 0 {})
            ]
            func [ "str" () StringType])
    ), Some(TypeCheckError::ReturnError(
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

#[test]
fn print_value_error() {
    assert_eq!(type_check(ResolvedAst {
        ast: vec![
            Statement::Print {
                value: Expression::Number {value: 2}
            }
        ],
        symtable: Symtable {
            variable_table: vec![],
            function_table: vec![]
        },
        error_mode: false
    }), Some(TypeCheckError::MismatchedTypeError(
        TypeError::PrintValueError(Type::Integer)
    )));
}

#[test]
fn variable_assignment_error() {
    assert_eq!(type_check(ResolvedAst {
        ast: vec![
            Statement::VariableAssignment {
                value: Expression::Number {value: 2},
                identifier: 0
            }
        ],
        symtable: Symtable {
            variable_table: vec![
                LocalVariable {
                    index: 0,
                    vartype: Some(Type::StringType),
                    identifier: "error".to_string()
                }
            ],
            function_table: vec![]
        },
        error_mode: false
    }), Some(TypeCheckError::MismatchedTypeError(
        TypeError::VariableAssignmentError(
            "error".to_string(),
            Type::StringType,
            Type::Integer
        )
    )));
}

#[test]
fn returned_value_error() {
    assert_eq!(type_check(ResolvedAst {
        ast: vec![
            Statement::FunctionDefinition {
                identifier: 0,
                block: Box::new(vec![
                    Statement::ResolvedReturn {
                        function: 0,
                        return_value: Some(Expression::StringValue {
                            value: "a".to_string()
                        })
                    }
                ])
            }
        ],
        symtable: Symtable {
            variable_table: vec![],
            function_table: vec![
                Function {
                    return_type: Some(Type::Integer),
                    param_list: vec![],
                    identifier: "error".to_string()
                }
            ]
        },
        error_mode: false
    }), Some(TypeCheckError::MismatchedTypeError(
        TypeError::ReturnedValueError(
            "error".to_string(),
            Type::Integer,
            Type::StringType
        )
    )));
}

#[test]
fn parameter_argument_error() {
    assert_eq!(type_check(ResolvedAst {
        ast: vec![
            Statement::FunctionDefinition {
                identifier: 0,
                block: Box::new(vec![
                    Statement::ResolvedReturn {
                        function: 0,
                        return_value: None
                    }
                ])
            },
            Statement::Call {
                expression: Expression::ResolvedFunctionCall {
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
            variable_table: vec![
                LocalVariable {
                    index: 0,
                    identifier: "error".to_string(),
                    vartype: Some(Type::Integer)
                }
            ],
            function_table: vec![
                Function {
                    return_type: None,
                    param_list: vec![
                        ParamType {
                            identifier: "error".to_string(),
                            param_type: Type::Integer
                        }
                    ],
                    identifier: "error".to_string()
                }
            ]
        },
        error_mode: false
    }), Some(TypeCheckError::MismatchedTypeError(
        TypeError::ParameterArgumentError(
            "error".to_string(),
            "error".to_string(),
            Type::Integer,
            Type::StringType
        )
    )));
}

#[test]
fn binary_operand_error() {
    assert_eq!(type_check(ResolvedAst {
        ast: vec![
            Statement::Call {
                expression: Expression::BinaryOperation {
                    op_type: OperationType::Add,
                    lhs: Box::new(Expression::Number {
                        value: 2
                    }),
                    rhs: Box::new(Expression::StringValue {
                        value: "a".to_string()
                    })
                }
            }
        ],
        symtable: Symtable {
            variable_table: vec![],
            function_table: vec![]
        },
        error_mode: false
    }), Some(TypeCheckError::MismatchedTypeError(
        TypeError::BinaryOperandError(
            OperationType::Add,
            Type::Integer,
            Type::Integer,
            Type::StringType
        )
    )));
}
