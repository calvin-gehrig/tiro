use crate::compiler::CompiledProgram;
use crate::comp_prog;

use super::{
    Interpreter,
    interpret_code
};

use crate::bytecode::{
    Op,
    NumSize
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
    assert_eq!(interpret(
        comp_prog!(
            main [ Push, val8 0, Write]
            stri ["a"])
    ),
    vec!["a"]);
}

#[test]
fn variable_declaration() {
    assert_eq!(interpret(
        comp_prog!(
            main [ Push, val8 0, Load, val8 0, val8 0, Write]
            stri ["a"])
    ),
    vec!["a"]);
}

#[test]
fn function_declaration() {
    assert_eq!(interpret(
        comp_prog!(
            main [
                Push, val8 0, Call, val8 0, val8 1,
                Push, val8 1, Call, val8 1, val8 2, Pop
            ]
            stri ["pff", "aah"]
            func [
                {
                    Load, val8 0, val8 0, Write,
                    Load, val8 0, val8 0, Return
                },
                {
                    Load, val8 0, val8 0, Write,
                    Load, val8 1, val8 0, Write,
                    Return
                }
            ])
    ), vec!["pff", "pff", "aah"]);
}

#[test]
fn concat_operation() {
    assert_eq!(interpret(
        comp_prog!(
            main [ Push, val8 0, Push, val8 1, Cat, Write]
            stri ["b", "a"])
    ),
    vec!["ab"]);
}

#[test]
fn str_int_cast() {
    assert_eq!(interpret(
            comp_prog!(
                main [ 
                    Push, val8 0, Str2Int, Push, val8 2,
                    Add, Int2Str, Write
                ]
                stri ["3"])
    ),
    vec!["5"]);
}

#[test]
fn int_bool_cast() {
    assert_eq!(interpret(
            comp_prog!(
                main [ Push, val8 5, Int2Bool, Int2Str, Write])
    ),
    vec!["1"]);
}

#[test]
fn str_bool_cast() {
    assert_eq!(interpret(
            comp_prog!(
                main [ Push, val8 0, Str2Bool, Bool2Str, Write]
                stri [""])
    ),
    vec!["false"]);
}
