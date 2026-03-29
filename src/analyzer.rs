use crate::parser::ast::Statement;

pub mod resolver;
use resolver::{
    resolve,
    Symtable
};

pub mod type_checker;
use type_checker::type_check;

pub struct AnalyzedAst {
    pub symtable: Symtable,
    pub ast: Vec<Statement>
}

pub fn analyze(ast: Vec<Statement>) -> AnalyzedAst {
    let resolved_ast = resolve(ast);
    let analyzed_ast = type_check(resolved_ast);
    if analyzed_ast.error_mode {
        panic!("Compilation error");
    }
    AnalyzedAst {
        ast: analyzed_ast.ast,
        symtable: analyzed_ast.symtable
    }
}
