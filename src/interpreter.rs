use std::vec::IntoIter;

use crate::compiler::CompiledProgram;

use crate::bytecode::{
    Op,
    Frame,
    NumSize
};

//#[cfg(test)]
//mod tests;

struct Interpreter {
    main_program: IntoIter<u8>,
    function_pool: Vec<IntoIter<u8>>,
    string_pool: Vec<String>,
    stack: Vec<usize>,
    output: Vec<String>,
    current_frame: usize,
    current_function: usize
}

impl Interpreter {
    fn new(program: CompiledProgram) -> Self {
        Self {
            main_program: program.main_program.into_iter(),
            function_pool: program.function_pool.into_iter().map(
                |function| {
                    function.into_iter()
                }).collect(),
            string_pool: program.string_pool,
            stack: vec![],
            output: Vec::new(),
            current_frame: 0,
            current_function: 0
        }
    }

    fn next_opcode(&mut self) -> Option<Op> {
        if let Some(byte) = self.next_byte() {
            Some(Op::from(byte))
        } else { None }
    }

    fn next_byte(&mut self) -> Option<u8> {
        match self.current_function {
            0 => self.main_program.next(),
            id => self.function_pool[id - 1].next()
        }
    }

    fn read_value(&mut self) -> usize {
        if let Some(byte) = self.next_byte() {
            match NumSize::from(byte) {
                NumSize::_8 => usize::from(self.next_byte().unwrap()),
                NumSize::_32 => {
                    let mut bytes: [u8; 4] = [0; 4];
                    for i in 0..3 {
                        bytes[i] = self.next_byte().unwrap();
                    }
                    usize::try_from(u32::from_le_bytes(bytes))
                        .unwrap()
                },
                NumSize::_64 => {
                    let mut bytes: [u8; 8] = [0; 8];
                    for i in 0..7 {
                        bytes[i] = self.next_byte().unwrap();
                    }
                    usize::try_from(u64::from_le_bytes(bytes))
                        .unwrap()
                }
            }
        } else { panic!("Tried to read value at end of program") }
    }

    fn push_stack(&mut self, stack_value: usize) {
        self.stack.push(stack_value);
    }

    fn pop_stack(&mut self) -> usize {
        self.stack.pop().expect("Unexpected empty stack")
    }

    fn get_stack(&mut self, index: usize, depth: usize) {
        let mut varframe = self.current_frame;
        for _ in 0..depth {
            varframe = self.stack[varframe];
        }
        self.push_stack(self.stack[varframe + 1 + index].clone());
    }

    fn get_string(&self, index: usize) -> String {
        self.string_pool[index].clone()
    }

    fn write_string(&mut self, string: String) -> usize {
        self.string_pool.push(string);
        self.string_pool.len() - 1
    }

    fn write(&mut self, value: String) {
        self.output.push(value);
    }

    fn call(&mut self, id: usize) {
        self.stack.push(self.current_function);
        self.stack.push(0x01);
        self.current_function = id + 1;
        self.stack.push(self.current_frame);
        self.current_frame = self.stack.len() - 1;
    }

    fn unwind(&mut self) {
        let mut varframe = self.current_frame;
        loop {
            match Frame::from(self.stack[varframe - 1]) {
                Frame::Func => {
                    self.current_function = self.stack[varframe - 2];
                    self.current_frame = self.stack[varframe];
                    self.stack.truncate(varframe - 1);
                    break;
                },
                Frame::Block => {
                    varframe = self.stack[varframe];
                }
            }
        }
    }

    fn output(self) -> Vec<String> {
        self.output
    }
}

pub fn interpret(program: CompiledProgram) {
    let mut interpreter = Interpreter::new(program);
    while let Some(opcode) = interpreter.next_opcode() {
        interpret_code(opcode, &mut interpreter);
    }
    for line in interpreter.output() {
        println!("{}", line);
    }
}

fn interpret_code(opcode: Op, interpreter: &mut Interpreter) {
    match opcode {
        Op::Push => {
            let value = interpreter.read_value();
            interpreter.push_stack(value);
        },
        Op::Pop => {
            interpreter.pop_stack();
        },
        Op::Call => {
            let id = interpreter.read_value();
            let arity = interpreter.read_value();

            let mut args = Vec::new();
            for _ in 0..arity {
                args.push(interpreter.pop_stack());
            }

            interpreter.call(id);
            args.into_iter().rev().for_each(|arg| {
                interpreter.push_stack(arg);
            });
        },
        Op::Return => {
            let return_value = interpreter.pop_stack();
            interpreter.unwind();
            interpreter.push_stack(return_value);
        },
        Op::Write => {
             let index = interpreter.pop_stack();
            interpreter.write(
                interpreter.get_string(index));
        },
        Op::Load => {
            let index = interpreter.read_value();
            let depth = interpreter.read_value();
            interpreter.get_stack(index, depth);
        },
        Op::Add => {
            let lhs = interpreter.pop_stack();
            let rhs = interpreter.pop_stack();
            interpreter.push_stack(lhs + rhs);
        },
        Op::Sub => {
            let lhs = interpreter.pop_stack();
            let rhs = interpreter.pop_stack();
            interpreter.push_stack(lhs - rhs);
        },
        Op::Mul => {
            let lhs = interpreter.pop_stack();
            let rhs = interpreter.pop_stack();
            interpreter.push_stack(lhs * rhs);
        },
        Op::Div => {
            let lhs = interpreter.pop_stack();
            let rhs = interpreter.pop_stack();
            interpreter.push_stack(lhs / rhs);
        },
        Op::Pow => {
            let lhs = interpreter.pop_stack();
            let rhs = interpreter.pop_stack();
            interpreter.push_stack(lhs ^ rhs);
        },
        Op::Cat => {
            let lhs_index = interpreter.pop_stack();
            let lhs = interpreter.get_string(lhs_index);

            let rhs_index = interpreter.pop_stack();
            let rhs = interpreter.get_string(rhs_index);

            let index = interpreter.write_string(lhs + &rhs);
            interpreter.push_stack(index);
        },
    }
}
