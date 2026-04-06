use crate::common::{
    ResolvedAst,
    Symtable,
    Statement,
    Expression,
    Parameter,
    ParamType,
    OperationType,
    Function,
    LocalVariable,
    Type
};
use crate::{
    opt,
    stmt,
    expr,
    res_ast
};

use super::{
    Resolver,
    resolve,
    resolve_block,
    register_global,
    error::ReferenceError
};

fn resolve_error(ast: Vec<Statement>) -> Vec<ReferenceError> {
    let mut resolver = Resolver::new();
    register_global(&ast, &mut resolver);
    resolve_block(ast, &mut resolver);
    resolver.error_stack
}

#[test]
fn variable() {
    assert_eq!(resolve(vec![
        stmt!(decl "a" "cat" expr!(stri "a")),
        stmt!(print expr!(var "a"))
    ]), res_ast!(
            ast [
                stmt!(assign 0 expr!(stri "a")),
                stmt!(print expr!(loc 0 0))
            ]
            var ["a" 0 StringType]
    ));
}

#[test]
fn function() {
    assert_eq!(resolve(vec![
        stmt!(func "select" ("a" "cat", "b" "cat") "cat" {}
            expr!(var "b")
        ),
        stmt!(call expr!(fncall "select" (
                    expr!(stri "pff"),
                    expr!(stri "aah")
        )))
    ]), res_ast!(
        ast [
            stmt!(def 0 {} expr!(loc 1 0)),
            stmt!(call expr!(rescall 0 (
                        expr!(stri "pff"),
                        expr!(stri "aah")
            )))
        ]
        var [ 
            "a" 0 StringType,
            "b" 1 StringType
        ]
        func [ "select" (
                "a" StringType,
                "b" StringType
            ) StringType
        ]
    ));
}

#[test]
fn recursive_function() {
    assert_eq!(resolve(vec![
        stmt!(func "recur" () {} expr!(fncall "recur" ())),
        stmt!(call expr!(fncall "recur" ()))
    ]), res_ast!(
        ast [
            stmt!(def 0 {} expr!(rescall 0 ())),
            stmt!(call expr!(rescall 0 ()))
        ]
        func [ "recur" ()]
    ));
}

#[test]
fn binary_operation() {
    assert_eq!(resolve(vec![
            stmt!(decl "a" expr!(num 2)),
            stmt!(decl "b" expr!(num 3)),
            stmt!(call expr!(bin Add
                    expr!(var "a"),
                    expr!(var "b")
            ))
        ]), res_ast!(
            ast [
                stmt!(assign 0 expr!(num 2)),
                stmt!(assign 1 expr!(num 3)),
                stmt!(call expr!(bin Add
                        expr!(loc 0 0),
                        expr!(loc 1 0)
                ))
            ]
            var [ "a" 0, "b" 1]
        ));
}


#[test]
fn variable_error() {
    assert_eq!(resolve_error(vec![
            stmt!(print expr!(var "a"))
    ]), vec![ReferenceError::UndefinedVariableName("a".to_string())]);
}

#[test]
fn symbol_as_variable_error() {
    assert_eq!(resolve_error(vec![
            stmt!(func "error" () {}),
            stmt!(call expr!(var "error"))
    ]), vec![ReferenceError::InvalidSymbolUseAsVariable("error".to_string())]);
}

#[test]
fn function_error() {
    assert_eq!(resolve_error(vec![
            stmt!(call expr!(fncall "error" ()))
    ]), vec![ReferenceError::UndefinedFunctionName("error".to_string())]);
}

#[test]
fn symbol_as_function_error() {
    assert_eq!(resolve_error(vec![
            stmt!(decl "a" "cat" expr!(stri "a")),
            stmt!(call expr!(fncall "a" ()))
    ]), vec![ReferenceError::InvalidSymbolUseAsFunction("a".to_string())]);
}

#[test]
fn type_error() {
    assert_eq!(resolve_error(vec![
            stmt!(decl "a" "ouh" expr!(stri "a"))
    ]), vec![ReferenceError::UndefinedTypeName("ouh".to_string())]);
}
