use crate::parser::ast::{
    Statement,
    Expression
};

#[cfg(test)]
mod tests;

struct Compiler {
    bytecode: Vec<Opcode>,
    heap: Vec<HeapValue>
}

impl Compiler {
    fn new () -> Self {
        Self {
            bytecode: Vec::new(),
            heap: Vec::new()
        }
    }

    fn push_opcode(&mut self, opcode: Opcode) {
        self.bytecode.push(opcode);
    }

    fn push_heap(&mut self, value: HeapValue) -> usize {
        self.heap.push(value);
        self.heap.len() - 1
    }

    fn return_bytecode(self) -> (Vec<HeapValue>, Vec<Opcode>) {
        (self.heap, self.bytecode)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    Push(StackValue),
    Print
}

#[derive(Debug, Clone, PartialEq)]
pub enum StackValue {
    HeapIndex(usize)
}

#[derive(Debug, Clone, PartialEq)]
pub enum HeapValue {
    StringValue(String)
}

pub fn compile(ast: Vec<Statement>) -> (Vec<HeapValue>, Vec<Opcode>) {
    let mut compiler = Compiler::new();
    for statement in ast {
        compile_statement(statement, &mut compiler);
    }
    compiler.return_bytecode()
}

fn compile_statement(statement: Statement, compiler: &mut Compiler) {
    match statement {
        Statement::Print {value} => compile_print(value, compiler),
    }
}

fn compile_print(value: Expression, compiler: &mut Compiler) {
    compile_expression(value, compiler);
    compiler.push_opcode(Opcode::Print);
}

fn compile_expression(expression: Expression, compiler: &mut Compiler) {
    match expression {
        Expression::StringValue {value} => compile_string(value, compiler),
    }
}

fn compile_string(value: String, compiler: &mut Compiler) {
    let index = compiler.push_heap(HeapValue::StringValue(value));
    compiler.push_opcode(Opcode::Push(StackValue::HeapIndex(index)));
}
