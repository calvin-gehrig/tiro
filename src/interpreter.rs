use crate::compiler::{
    StackValue,
    HeapValue,
    Opcode
};

#[cfg(test)]
mod tests;

struct Interpreter {
    bytecode: std::vec::IntoIter<Opcode>,
    heap: Vec<HeapValue>,
    stack: Vec<StackValue>,
    output: Vec<String>
}

impl Interpreter {
    fn new(bytecode: Vec<Opcode>, heap: Vec<HeapValue>) -> Self {
        Self {
            bytecode: bytecode.into_iter(),
            stack: Vec::new(),
            output: Vec::new(),
            heap
        }
    }

    fn next_opcode(&mut self) -> Option<Opcode> {
        self.bytecode.next()
    }

    fn push_stack(&mut self, stack_value: StackValue) {
        self.stack.push(stack_value);
    }

    fn pop_stack(&mut self) -> StackValue {
        self.stack.pop().expect("Unexpected empty stack")
    }

    fn get_heap(&mut self, index: usize) -> HeapValue {
        self.heap[index].clone()
    }

    fn print(&mut self, string: String) {
        self.output.push(string);
    }

    fn output(self) -> Vec<String> {
        self.output
    }
}

pub fn interpret((heap, bytecode): (Vec<HeapValue>, Vec<Opcode>)) {
    let mut interpreter = Interpreter::new(bytecode, heap);
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
        Opcode::Print => interpret_print(interpreter)
    }
}

fn interpret_print(interpreter: &mut Interpreter) {
    match interpreter.pop_stack() {
        StackValue::HeapIndex(index) => match interpreter.get_heap(index) {
            HeapValue::StringValue(string) => interpreter.print(string),
        },
    }
}
