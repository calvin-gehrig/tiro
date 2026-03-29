use crate::parser::ast::{
    Statement, 
    Expression,
    Symbol
};

use crate::analyzer::{
    AnalyzedAst,
    resolver::Symtable,
    type_checker::TiroType
};

use super::{
    compile,
    Opcode,
    HeapValue,
    StackValue,
    LocalVar
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
            symtable: Symtable {variable_table:vec![]}
        }), (vec![HeapValue::StringValue("a".to_string())], vec![
            Opcode::Push(StackValue::HeapIndex(0)),
            Opcode::Print
    ]));
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
                    identifier: Symbol::Id(0)
                },
                Statement::Print {
                    value:
                        Expression::Variable {
                            identifier: Symbol::Id(0)
                        }
                }
            ],
            symtable: Symtable {variable_table:vec![Some(TiroType::StringType)]}
        }), (vec![HeapValue::StringValue("a".to_string())], vec![
            Opcode::Push(StackValue::HeapIndex(0)),
            Opcode::Push(StackValue::LocalVar(LocalVar {
                index: 0,
                depth: 0
            })),
            Opcode::Print
    ]));
}
