use crate::common::{
    AnalyzedAst,
    Symtable,
    Statement, 
    Expression,
    Function,
    LocalVariable,
    ParamType,
    OperationType,
    Type
};
use crate::{
    opt,
    stmt,
    expr,
    an_ast,
    comp_prog
};

use super::{
    compile,
    CompiledProgram
};

use crate::bytecode::{
    Op,
    NumSize
};

#[test]
fn print() {
    assert_eq!(compile(
        an_ast!(
            ast [
                stmt!(print expr!(stri "a"))
            ]),
        ), comp_prog!(
            main [Push, val8 0, Write]
            stri ["a"])
    );
}

#[test]
fn variable_declaration() {
    assert_eq!(compile(
        an_ast!(
            ast [
                stmt!(assign 0 expr!(stri "a")),
                stmt!(print expr!(loc 0 0))
            ]
            var [ "a" 0 StringType])
        ), comp_prog!(
            main [Push, val8 0, Load, val8 0, val8 0, Write]
            stri ["a"])
    );
}

#[test]
fn function_declaration() {
    assert_eq!(compile(
        an_ast!(
            ast [
                stmt!(def 0 {} expr!(loc 0 0)),
                stmt!(call expr!(rescall 0 (expr!(stri "a"))))
            ]
            var [ "a" 0 StringType]
            func ["id" ("a" StringType) StringType])
        ), comp_prog!(
            main [ Push, val8 0, Call, val8 0, val8 1, Pop]
            stri ["a"]
            func [ { Load, val8 0, val8 0, Return } ])
    );
}

#[test]
fn math_operation() {
    assert_eq!(compile(
        an_ast!(
            ast [
                stmt!(call expr!(bin Add
                        expr!(bin Sub
                            expr!(bin Mul
                                expr!(num 2),
                                expr!(num 3)
                            ),
                            expr!(num 5)
                        ),
                        expr!(bin Div
                            expr!(bin Pow
                                expr!(num 2),
                                expr!(num 4)
                            ),
                            expr!(num 6)
                        )
                ))
            ])
        ), comp_prog!(
            main [
                Push, val8 6, Push, val8 4, Push, val8 2,
                Pow, Div, Push, val8 5, Push, val8 3, Push, val8 2,
                Mul, Sub, Add, Pop
            ])
    );
}

#[test]
fn concat() {
    assert_eq!(compile(
        an_ast!(
            ast [
                stmt!(call expr!(bin Cat
                        expr!(stri "a"),
                        expr!(stri "b")
                ))
            ]),
        ), comp_prog!(
            main [ Push, val8 0, Push, val8 1, Cat, Pop ]
            stri ["b", "a"])
    );
}

#[test]
fn cast() {
    assert_eq!(compile(
        an_ast!(
            ast [
                stmt!(call expr!(acast
                        Integer StringType
                        expr!(num 5)
                ))
            ]),
        ), comp_prog!(
            main [ Push, val8 5, Int2Str, Pop ])
    );
}
