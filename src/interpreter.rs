use std::vec::IntoIter;

use crate::compiler::{
    StackValue,
    Opcode,
    CompiledProgram
};

#[cfg(test)]
mod tests;

struct Interpreter {
    main_program: IntoIter<Opcode>,
    function_pool: Vec<IntoIter<Opcode>>,
    string_pool: Vec<String>,
    stack: Vec<StackValue>,
    output: Vec<String>,
    current_frame: usize,
    current_function: Option<usize>
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
            stack: vec![StackValue::EOS],
            output: Vec::new(),
            current_frame: 0,
            current_function: None
        }
    }

    fn next_opcode(&mut self) -> Option<Opcode> {
        match self.current_function {
            None => self.main_program.next(),
            Some(id) => self.function_pool[id].next()
        }
    }

    fn push_stack(&mut self, stack_value: StackValue) {
        self.stack.push(stack_value);
    }

    fn pop_stack(&mut self) -> StackValue {
        self.stack.pop().expect("Unexpected empty stack")
    }

    fn get_stack(&mut self, index: usize, depth: usize) {
        let mut var_frame = self.current_frame;
        for _ in 0..depth {
            if let StackValue::UpperFrame(upper_frame) = self.stack[self.current_frame] {
                var_frame = upper_frame;
            } else { panic!("Unexpected stackvalue instead of frame") }
        }
        self.push_stack(self.stack[var_frame + 1 + index].clone());
    }

    fn get_string(&self, index: usize) -> String {
        self.string_pool[index].clone()
    }

    fn print(&mut self, value: ExprValue) {
        let string_value = match value {
            ExprValue::StringValue(string) => string
        };
        self.output.push(string_value);
    }

    fn call(&mut self, id: usize) {
        self.stack.push(StackValue::UpperFunction(self.current_function));
        self.current_function = Some(id);
        self.stack.push(StackValue::UpperFrame(self.current_frame));
        self.current_frame = self.stack.len() - 1;
    }

    fn unwind(&mut self) {
        let mut varframe = self.current_frame;
        loop {
            if let StackValue::UpperFunction(maybe_id) = self.stack[varframe - 1] {
                self.current_function = maybe_id;
                if let StackValue::UpperFrame(frame) = self.stack[varframe] {
                    self.current_frame = frame;
                    self.stack.truncate(varframe - 1);
                }
                break;
            } else if let StackValue::UpperFrame(frame) = self.stack[varframe] {
                varframe = frame;
            }
        }
    }

    fn output(self) -> Vec<String> {
        self.output
    }
}

enum ExprValue {
    StringValue(String)
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

fn interpret_code(opcode: Opcode, interpreter: &mut Interpreter) {
    match opcode {
        Opcode::Push(stack_value) => interpreter.push_stack(stack_value),
        Opcode::Pop => {interpreter.pop_stack();},
        Opcode::Call(id, arity) => interpret_call(id, arity, interpreter),
        Opcode::Return => interpret_return(interpreter),
        Opcode::Print => interpret_print(interpreter),
        Opcode::LoadLocal(index, depth) => interpreter.get_stack(index, depth)
    }
}

fn interpret_print(interpreter: &mut Interpreter) {
    let value = interpret_expression(interpreter.pop_stack(), interpreter);
    interpreter.print(value);
}

fn interpret_call(id: usize, arity: usize, interpreter: &mut Interpreter) {
    let mut args = Vec::new();
    for _ in 0..arity {
        args.push(interpreter.pop_stack());
    }
    interpreter.call(id);
    args.into_iter().rev().for_each(|arg| {
        interpreter.push_stack(arg);
    });
}

fn interpret_return(interpreter: &mut Interpreter) {
    let return_value = interpreter.pop_stack();
    interpreter.unwind();
    interpreter.push_stack(return_value);
}

fn interpret_expression(expression: StackValue, interpreter: &mut Interpreter) -> ExprValue {
    match expression {
        StackValue::StringIndex(index) => ExprValue::StringValue(
            interpreter.get_string(index)
        ),
        _ => panic!("Unexpected stack value {:?}", expression)
    }
}
