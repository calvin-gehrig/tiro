use crate::compiler::{
    StackValue,
    HeapValue,
    Opcode,
    LocalVar
};

#[cfg(test)]
mod tests;

struct Interpreter {
    bytecode: std::vec::IntoIter<Opcode>,
    heap: Vec<HeapValue>,
    stack: Vec<StackValue>,
    output: Vec<String>,
    current_frame: usize
}

impl Interpreter {
    fn new(bytecode: Vec<Opcode>, heap: Vec<HeapValue>) -> Self {
        Self {
            bytecode: bytecode.into_iter(),
            stack: vec![StackValue::EOS],
            output: Vec::new(),
            current_frame: 0,
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

    fn get_stack(&self, index: usize, depth: usize) -> StackValue {
        let mut var_frame = self.current_frame;
        for _ in 0..depth {
            if let StackValue::UpperFrame(upper_frame) = self.stack[self.current_frame] {
                var_frame = upper_frame;
            } else { panic!("Unexpected stackvalue instead of frame") }
        }
        self.stack[var_frame + 1 + index].clone()
    }

    fn get_heap(&mut self, index: usize) -> HeapValue {
        self.heap[index].clone()
    }

    fn print(&mut self, value: ExprValue) {
        let string_value = match value {
            ExprValue::StringValue(string) => string
        };
        self.output.push(string_value);
    }

    fn output(self) -> Vec<String> {
        self.output
    }
}

enum ExprValue {
    StringValue(String)
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
    let value = interpret_expression(interpreter.pop_stack(), interpreter);
    interpreter.print(value);
}

fn interpret_expression(expression: StackValue, interpreter: &mut Interpreter) -> ExprValue {
    match expression {
        StackValue::HeapIndex(index) => interpret_heap_index(index, interpreter),
        StackValue::LocalVar(local_var) => interpret_local_var(local_var, interpreter),
        _ => panic!("Unexpected stack value {:?}", expression)
    }
}

fn interpret_heap_index(index: usize, interpreter: &mut Interpreter) -> ExprValue {
    match interpreter.get_heap(index) {
        HeapValue::StringValue(string) => ExprValue::StringValue(string)
    }
}

fn interpret_local_var(local_var: LocalVar, interpreter: &mut Interpreter) -> ExprValue {
    interpret_expression(
        interpreter.get_stack(local_var.index, local_var.depth),
        interpreter)
}
