use crate::compiler::{
    Opcode,
    StackValue,
    CompiledProgram
};

use super::{
    Interpreter,
    interpret_code
};

fn interpret(program: CompiledProgram) -> Vec<String> {
    let mut interpreter = Interpreter::new(program);
    while let Some(opcode) = interpreter.next_opcode() {
        interpret_code(opcode, &mut interpreter);
    }
    interpreter.output()
}

#[test]
fn print() {
    assert_eq!(interpret(CompiledProgram {
            main_program: vec![
                Opcode::Push(StackValue::StringIndex(0)),
                Opcode::Print
            ],
            string_pool: vec!["a".to_string()],
            function_pool: vec![]
        }),
        vec!["a".to_string()]);
}

#[test]
fn variable_declaration() {
    assert_eq!(interpret(CompiledProgram {
            main_program: vec![
                Opcode::Push(StackValue::StringIndex(0)),
                Opcode::Push(StackValue::LocalVar(0, 0)),
                Opcode::Print
            ],
            string_pool: vec!["a".to_string()],
            function_pool: vec![]
        }),
        vec!["a"]);
}
