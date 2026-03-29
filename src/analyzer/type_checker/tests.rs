use crate::parser::ast::{
    Statement,
    Expression,
    Symbol
};

use super::{
    type_check,
    TiroType
};

use crate::analyzer::resolver::{
    ResolvedAst,
    Symtable
};

#[test]
fn print () {
    assert_eq!(type_check(ResolvedAst {
            ast: vec![
                Statement::Print {
                    value: Expression::StringValue {value: "a".to_string()}
                }
            ],
            symtable: Symtable {
                variable_table: vec![]
            },
            error_mode: false
    }).error_mode, false);
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
                variable_table: vec![Some(TiroType::StringType)]
            },
            error_mode: false
    }).error_mode, false);
}
