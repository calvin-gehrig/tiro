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
                Opcode::LoadLocal(0, 0),
                Opcode::Print
            ],
            string_pool: vec!["a".to_string()],
            function_pool: vec![]
        }),
        vec!["a"]);
}

#[test]
fn function_declaration() {
    assert_eq!(interpret(CompiledProgram {
        main_program: vec![
            Opcode::Push(StackValue::StringIndex(0)),
            Opcode::Call(0, 1),
            Opcode::Push(StackValue::StringIndex(1)),
            Opcode::Call(1, 2),
            Opcode::Pop
        ],
        string_pool: vec![
            "pff".to_string(),
            "aah".to_string()
        ],
        function_pool: vec![
            vec![
                Opcode::LoadLocal(0, 0),
                Opcode::Print,
                Opcode::LoadLocal(0, 0),
                Opcode::Return
            ],
            vec![
                Opcode::LoadLocal(0, 0),
                Opcode::Print,
                Opcode::LoadLocal(1, 0),
                Opcode::Print,
                Opcode::Return
            ]
        ]
    }),
    vec!["pff", "pff", "aah"]);
}
