use crate::common::{
    AnalyzedAst,
    Symtable,
    Statement, 
    Expression,
    Symbol,
    TiroType
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
            symtable: Symtable {
                variable_table:vec![],
                function_table:vec![]
            }
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
            symtable: Symtable {
                variable_table:vec![Some(TiroType::StringType)],
                function_table:vec![]
            }
        }), (vec![HeapValue::StringValue("a".to_string())], vec![
            Opcode::Push(StackValue::HeapIndex(0)),
            Opcode::Push(StackValue::LocalVar(LocalVar {
                index: 0,
                depth: 0
            })),
            Opcode::Print
    ]));
}
