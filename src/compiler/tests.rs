use crate::parser::ast::{
    Statement, 
    Expression
};

use super::{
    compile,
    Opcode,
    HeapValue,
    StackValue
};

#[test]
fn print() {
    assert_eq!(compile(vec![
        Statement::Print {
            value:
                Expression::StringValue {
                    value: "a".to_string()
                }
        }]), (vec![HeapValue::StringValue("a".to_string())], vec![
            Opcode::Push(StackValue::HeapIndex(0)),
            Opcode::Print
        ]));
}
