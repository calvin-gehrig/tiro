use crate::common::{
    Statement,
    Expression,
    OperationType,
    Parameter
};
use crate::{
    stmt,
    expr
};

use crate::lexer::Lexer;

use super::parse;

fn parse_test(src: &str) -> Vec<Statement> {
    let lexer = Lexer::new(&src);
    parse(lexer)
}

#[test]
fn print_string() {
    assert_eq!(parse_test("echo \"a\""), vec![
        stmt!(print expr!(stri "a"))
    ]);
}

#[test]
fn variable_declaration() {
    assert_eq!(parse_test("sit _a1:cat \"a\""), vec![
        stmt!(decl "_a1" "cat" expr!(stri "a"))
    ]);
}

#[test]
fn print_variable() {
    assert_eq!(parse_test("echo a"), vec![
        stmt!(print expr!(var "a"))
    ]);
}

#[test]
fn function_declaration() {
    assert_eq!(parse_test("fvnctio greet() echo \"Hello\" reddi"), vec![
        stmt!(func "greet" () {
            stmt!(print expr!(stri "Hello"))
        })
    ]);
}

#[test]
fn str_id_declaration() {
    assert_eq!(parse_test("fvnctio str_id(a:cat, b:cat)=>cat reddi a"), 
        vec![ stmt!(func "str_id" ("a" "cat", "b" "cat") "cat" {}
            expr!(var "a"))
    ]);
}

#[test]
fn function_call() {
    assert_eq!(parse_test("voc a(\"a\")"), vec![
        stmt!(call expr!(fncall "a" (expr!(stri "a"))))
    ]);
}

#[test]
fn math_operation() {
    assert_eq!(parse_test("voc 3 + 2 * 4 / 6 - 1"), vec![
        stmt!(call expr!(bin Sub
                expr!(bin Add
                    expr!(num 3),
                    expr!(bin Div
                        expr!(bin Mul
                            expr!(num 2),
                            expr!(num 4)
                        ),
                        expr!(num 6)
                    )
                ),
                expr!(num 1)
        ))
    ]);
}

#[test]
fn concat() {
    assert_eq!(parse_test("voc \"a\" ~ \"b\""),vec![
        stmt!(call expr!(bin Cat
                expr!(stri "a"),
                expr!(stri "b")
        ))
    ]);
}

#[test]
fn power() {
    assert_eq!(parse_test("voc 2^4^5 + 2"), vec![
        stmt!(call expr!(bin Add
                expr!(bin Pow
                    expr!(num 2),
                    expr!(bin Pow
                        expr!(num 4),
                        expr!(num 5)
                    )
                ),
                expr!(num 2)
        ))
    ]);
}

#[test]
#[should_panic(expected = "Parsing error: UnrecognizedToken")]
fn parser_error() {
    parse_test("echo echo");
}

#[test]
#[should_panic(expected = "Parsing error: User { error: InvalidToken")]
fn lexer_error() {
    parse_test("§");
}
