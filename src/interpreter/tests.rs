use crate::compiler::{
    Opcode,
    StackValue,
    HeapValue,
    LocalVar
};

use super::{
    Interpreter,
    interpret_code
};

fn interpret(heap: Vec<HeapValue>, bytecode: Vec<Opcode>) -> Vec<String> {
    let mut interpreter = Interpreter::new(bytecode, heap);
    while let Some(opcode) = interpreter.next_opcode() {
        interpret_code(opcode, &mut interpreter);
    }
    interpreter.output()
}

#[test]
fn print() {
    assert_eq!(interpret(vec![
            HeapValue::StringValue("a".to_string())
        ], vec![
            Opcode::Push(StackValue::HeapIndex(0)),
            Opcode::Print
        ]),
        vec!["a"]);
}

#[test]
fn variable_declaration() {
    assert_eq!(interpret(vec![
            HeapValue::StringValue("a".to_string())    
        ], vec![
            Opcode::Push(StackValue::HeapIndex(0)),
            Opcode::Push(StackValue::LocalVar(LocalVar {
                index: 0,
                depth: 0
            })),
            Opcode::Print
        ]),
        vec!["a"]);
}
