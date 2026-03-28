use crate::compiler::{
    Opcode,
    StackValue,
    HeapValue
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
