use std::collections::HashMap;
use std::mem;

use crate::common::{
    AnalyzedAst,
    Symtable,
    Statement,
    Expression,
    Function,
    LocalVariable,
    Type
};

#[cfg(test)]
mod tests;

struct Compiler {
    main_program: Vec<Opcode>,
    string_pool: Vec<String>,
    function_pool: Vec<Vec<Opcode>>,
    symtable: Symtable,
    current_function: Option<usize>,
    upper_functions: Vec<usize>
}

impl Compiler {
    fn new (symtable: Symtable) -> Self {
        Self {
            main_program: Vec::new(),
            string_pool: Vec::new(),
            function_pool: Vec::new(),
            current_function: None,
            upper_functions: Vec::new(),
            symtable
        }
    }

    fn push_opcode(&mut self, opcode: Opcode) {
        match self.current_function {
            None => self.main_program.push(opcode),
            Some(id) => self.function_pool[id].push(opcode)
        }
    }

    fn push_string(&mut self, string: String) -> usize {
        self.string_pool.push(string);
        self.string_pool.len() - 1
    }

    fn get_local(&self, id: usize) -> usize {
        let variable = &self.symtable.variable_table[id];
        variable.index
    }

    fn start_function(&mut self) {
        self.function_pool.push(Vec::new());
        if let Some(mut upper_function) = self.current_function {
            self.upper_functions.push(mem::take(&mut upper_function));
        }
        self.current_function = Some(self.function_pool.len() - 1);
    }

    fn end_function(&mut self) {
        self.current_function = self.upper_functions.pop();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    Push(StackValue),
    Pop,
    Call(usize, usize),
    Return,
    Print
}

#[derive(Debug, Clone, PartialEq)]
pub enum StackValue {
    StringIndex(usize),
    LocalVar(usize, usize),
    UpperFrame(usize),
    UpperFunction(Option<usize>),
    Null,
    EOS
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompiledProgram {
    pub main_program: Vec<Opcode>,
    pub function_pool: Vec<Vec<Opcode>>,
    pub string_pool: Vec<String>
}

pub fn compile(analyzed_ast: AnalyzedAst) -> CompiledProgram {
    let mut compiler = Compiler::new(analyzed_ast.symtable);
    for statement in analyzed_ast.ast {
        compile_statement(statement, &mut compiler);
    }
    CompiledProgram {
        main_program: compiler.main_program,
        function_pool: compiler.function_pool,
        string_pool: compiler.string_pool
    }
}

fn compile_statement(statement: Statement, compiler: &mut Compiler) {
    match statement {
        Statement::Print {value} => compile_print(value, compiler),
        Statement::VariableAssignment {value, ..} => compile_expression(value, compiler),
        Statement::FunctionDefinition {block, ..} => compile_function_definition(*block, compiler),
        Statement::ResolvedReturn { return_value,.. } => compile_return_statement(return_value, compiler),
        Statement::Call {expression} => compile_call(expression, compiler),
        _ => panic!("Unsupported statement type")
    }
}

fn compile_print(value: Expression, compiler: &mut Compiler) {
    compile_expression(value, compiler);
    compiler.push_opcode(Opcode::Print);
}

fn compile_function_definition(block: Vec<Statement>, compiler: &mut Compiler) {
    compiler.start_function();
    for statement in block {
        compile_statement(statement, compiler);
    }
    compiler.end_function();
}

fn compile_return_statement (return_value: Option<Expression>, compiler: &mut Compiler) {
    match return_value {
        Some(value) => compile_expression(value, compiler),
        None => compiler.push_opcode(Opcode::Push(StackValue::Null))
    }
    compiler.push_opcode(Opcode::Return);
}

fn compile_call (expression: Expression, compiler: &mut Compiler) {
    compile_expression(expression, compiler);
    compiler.push_opcode(Opcode::Pop);
}

fn compile_expression(expression: Expression, compiler: &mut Compiler) {
    match expression {
        Expression::StringValue {value} => compile_string(value, compiler),
        Expression::LocalVar {id, depth} => compile_variable(id, depth, compiler),
        Expression::ResolvedFunctionCall {id, argument_list} => compile_function_call(id, *argument_list, compiler),
        _ => panic!("Unsupported expression type")
    }
}

fn compile_string(string: String, compiler: &mut Compiler) {
    let index = compiler.push_string(string);
    compiler.push_opcode(Opcode::Push(StackValue::StringIndex(index)));
}

fn compile_variable(id: usize, depth: usize, compiler: &mut Compiler) {
        let index = compiler.get_local(id);
        compiler.push_opcode(Opcode::Push(StackValue::LocalVar(index, depth)));
}

fn compile_function_call(id: usize, argument_list: Vec<Expression>, compiler: &mut Compiler) {
    let arity = argument_list.len();
    for argument in argument_list {
        compile_expression(argument, compiler);
    }
    compiler.push_opcode(Opcode::Call(id, arity));
}
