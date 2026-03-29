use std::collections::HashMap;

use crate::parser::ast::{
    Statement,
    Expression,
    Symbol
};

use crate::analyzer::{
    AnalyzedAst,
    resolver::Symtable,
    type_checker::TiroType
};

#[cfg(test)]
mod tests;

enum UpperStackEnv {
    EOE,
    Env(Box<StackEnv>)
}

struct StackEnv {
    current_environment: HashMap<usize, usize>,
    local_count: usize,
    upper_environment: UpperStackEnv
}

impl StackEnv {
    fn push_local (&mut self, id: usize) {
        self.current_environment.insert(id, self.local_count);
        self.local_count += 1;
    }

    fn get_local (&self, id: usize, depth: usize) -> LocalVar {
        let maybe_index = self.current_environment.get(&id).copied();
        match maybe_index {
            None => if let UpperStackEnv::Env(upper_env) = &self.upper_environment {
                    upper_env.get_local(id, depth + 1)
                } else { panic!("Uninitialized local variable") },
            Some(index) => LocalVar { index, depth }
        }
    }
}

struct Compiler {
    bytecode: Vec<Opcode>,
    heap: Vec<HeapValue>,
    symtable: Symtable,
    local_env: StackEnv
}

impl Compiler {
    fn new (symtable: Symtable) -> Self {
        Self {
            bytecode: Vec::new(),
            heap: Vec::new(),
            local_env: StackEnv {
                current_environment: HashMap::new(),
                local_count: 0,
                upper_environment: UpperStackEnv::EOE
            },
            symtable
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

    fn register_local(&mut self, id: usize) {
        self.local_env.push_local(id);
    }

    fn get_local(&self, id: usize) -> LocalVar {
        self.local_env.get_local(id, 0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    Push(StackValue),
    Print
}

#[derive(Debug, Clone, PartialEq)]
pub enum StackValue {
    HeapIndex(usize),
    LocalVar(LocalVar),
    UpperFrame(usize),
    EOS
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalVar {
    pub index: usize,
    pub depth: usize
}

#[derive(Debug, Clone, PartialEq)]
pub enum HeapValue {
    StringValue(String)
}

pub fn compile(analyzed_ast: AnalyzedAst) -> (Vec<HeapValue>, Vec<Opcode>) {
    let mut compiler = Compiler::new(analyzed_ast.symtable);
    for statement in analyzed_ast.ast {
        compile_statement(statement, &mut compiler);
    }
    compiler.return_bytecode()
}

fn compile_statement(statement: Statement, compiler: &mut Compiler) {
    match statement {
        Statement::Print {value} => compile_print(value, compiler),
        Statement::VariableAssignment {
            value,
            identifier
        } => compile_variable_assignment(value, identifier, compiler),
        _ => panic!("Unsupported statement type")
    }
}

fn compile_print(value: Expression, compiler: &mut Compiler) {
    compile_expression(value, compiler);
    compiler.push_opcode(Opcode::Print);
}

fn compile_variable_assignment(value: Expression, identifier: Symbol, compiler: &mut Compiler) {
    compile_expression(value, compiler);
    if let Symbol::Id(id) = identifier {
        compiler.register_local(id);
    } else { panic!("Unexpected unresolved identifier") }
}

fn compile_expression(expression: Expression, compiler: &mut Compiler) {
    match expression {
        Expression::StringValue {value} => compile_string(value, compiler),
        Expression::Variable {identifier} => compile_variable(identifier, compiler),
        _ => panic!("Unsupported expression type")
    }
}

fn compile_string(value: String, compiler: &mut Compiler) {
    let index = compiler.push_heap(HeapValue::StringValue(value));
    compiler.push_opcode(Opcode::Push(StackValue::HeapIndex(index)));
}

fn compile_variable(identifier: Symbol, compiler: &mut Compiler) {
    if let Symbol::Id(id) = identifier {
        let local_var = compiler.get_local(id);
        compiler.push_opcode(Opcode::Push(StackValue::LocalVar(local_var)));
    } else { panic!("Unexpected unresolved identifier") }
}
