use std::mem;

use crate::common::{
    AnalyzedAst,
    Symtable,
    Statement,
    Expression,
    OperationType,
    Type
};

use crate::bytecode::{
    Op,
    NumSize
};

#[cfg(test)]
mod tests;

struct Compiler {
    main_program: Vec<u8>,
    string_pool: Vec<String>,
    function_pool: Vec<Vec<u8>>,
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

    fn push_opcode(&mut self, opcode: Op) {
        self.push_byte(opcode as u8);
    }

    fn push_byte(&mut self, byte: u8) {
        match self.current_function {
            None => self.main_program.push(byte),
            Some(id) => self.function_pool[id].push(byte)
        }
    }

    fn push_value(&mut self, value: usize) {
        if value < u8::MAX.into() {
            self.push_byte(NumSize::_8 as u8);
            self.push_byte(u8::try_from(value).unwrap());

        } else if value < u32::MAX.try_into().unwrap() {
            let bytes = u32::try_from(value).unwrap().to_ne_bytes();
            self.push_byte(NumSize::_32 as u8);
            bytes.into_iter().for_each(|b| self.push_byte(b));

        } else {
            let bytes = u64::try_from(value).unwrap().to_ne_bytes();
            self.push_byte(NumSize::_64 as u8);
            bytes.into_iter().for_each(|b| self.push_byte(b));
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
pub struct CompiledProgram {
    pub main_program: Vec<u8>,
    pub function_pool: Vec<Vec<u8>>,
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
    compiler.push_opcode(Op::Write);
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
        None => {
            compiler.push_opcode(Op::Push);
            compiler.push_byte(0x00);
        }
    }
    compiler.push_opcode(Op::Return);
}

fn compile_call (expression: Expression, compiler: &mut Compiler) {
    compile_expression(expression, compiler);
    compiler.push_opcode(Op::Pop);
}

fn compile_expression(expression: Expression, compiler: &mut Compiler) {
    match expression {
        Expression::StringValue {value} => compile_string(value, compiler),
        Expression::Number {value} => compile_number(value, compiler),
        Expression::Boolean {value} => compile_bool(value, compiler),
        Expression::LocalVar {id, depth} => compile_variable(id, depth, compiler),
        Expression::ResolvedFunctionCall {id, argument_list} => compile_function_call(id, *argument_list, compiler),
        Expression::BinaryOperation {lhs, rhs, op_type} => compile_binary(*lhs, *rhs, op_type, compiler),
        Expression::AnalyzedCast {operand, input_type, output_type} => compile_cast(*operand, input_type, output_type, compiler),
        _ => panic!("Unsupported expression type")
    }
}

fn compile_string(string: String, compiler: &mut Compiler) {
    let index = compiler.push_string(string);
    compiler.push_opcode(Op::Push);
    compiler.push_value(index);
}

fn compile_number(value: u32, compiler: &mut Compiler) {
    compiler.push_opcode(Op::Push);
    compiler.push_value(usize::try_from(value).unwrap());
}

fn compile_bool(value: bool, compiler: &mut Compiler) {
    compiler.push_opcode(Op::Push);
    compiler.push_value(if value { 1 } else { 0 });
}

fn compile_variable(id: usize, depth: usize, compiler: &mut Compiler) {
        let index = compiler.get_local(id);
        compiler.push_opcode(Op::Load);
        compiler.push_value(index);
        compiler.push_value(depth);
}

fn compile_function_call(id: usize, argument_list: Vec<Expression>, compiler: &mut Compiler) {
    let arity = argument_list.len();
    for argument in argument_list {
        compile_expression(argument, compiler);
    }
    compiler.push_opcode(Op::Call);
    compiler.push_value(id);
    compiler.push_value(arity);
}

fn compile_binary(lhs: Expression, rhs: Expression, op_type: OperationType, compiler: &mut Compiler) {
    compile_expression(rhs, compiler);
    compile_expression(lhs, compiler);
    compiler.push_opcode(match op_type {
        OperationType::Add => Op::Add,
        OperationType::Sub => Op::Sub,
        OperationType::Mul => Op::Mul,
        OperationType::Div => Op::Div,
        OperationType::Pow => Op::Pow,
        OperationType::Cat => Op::Cat,
    });
}

fn compile_cast(operand: Expression, input_type: Type, output_type: Type, compiler: &mut Compiler) {
    compile_expression(operand, compiler);
    if input_type != Type::Boolean || output_type != Type::Integer {
        compiler.push_opcode(match (input_type, output_type) {
            (Type::StringType, Type::Integer) => Op::Str2Int,
            (Type::Integer, Type::StringType) => Op::Int2Str,
            (Type::Boolean, Type::StringType) => Op::Bool2Str,
            (Type::Integer, Type::Boolean) => Op::Int2Bool,
            (Type::StringType, Type::Boolean) => Op::Str2Bool,
            _ => panic!("Unexpected type conversion")
        });
    }
}
