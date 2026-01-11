use std::fs;
use std::env;

mod lexer;
mod parser;

fn main() {
    let raw_program : String = match fs::read_to_string(
        env::args().skip(1).next().expect("Expected program to interpret")
    ) {
        Ok(raw_program) => raw_program,
        Err(error) => panic!("Can't read provided file: {}", error)
    };
    let tokens : Vec<lexer::Token> = lexer::tokenize(raw_program);
    println!("{:?}", tokens);
    let ast : Vec<parser::Statement> = parser::parse(tokens);
    println!("{:?}", ast);
}
