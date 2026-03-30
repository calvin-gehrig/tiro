use crate::common::{
    ResolvedAst,
    Symtable,
    Statement,
    Expression,
    Symbol,
    TiroType
};

use super::type_check;

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
                variable_table: vec![Some(TiroType::StringType)],
                function_table: vec![]
            },
            error_mode: false
    }).error_mode, false);
}
