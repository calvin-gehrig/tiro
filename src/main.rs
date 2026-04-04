use std::env;
use std::fs;

mod common;

mod lexer;
use lexer::Lexer;

mod parser;
use parser::parse;

mod analyzer;
use analyzer::analyze;

mod compiler;
use compiler::compile;

mod interpreter;
use interpreter::interpret;

fn main() {
    let src = fs::read_to_string(env::args().nth(1).expect("Expected file argument"))
        .expect("Failed to read file");
    let lexer = Lexer::new(&src);
    let ast = parse(lexer);
    let analyzed_ast = analyze(ast);
    let compiled_program = compile(analyzed_ast);
    interpret(compiled_program);
}
