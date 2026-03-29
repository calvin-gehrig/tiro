use crate::parser::ast::{
    Statement,
    Expression,
    Symbol
};

use crate::analyzer::resolver::{
    ResolvedAst,
    Symtable
};

mod error;
use error::{
    TypeCheckError,
    TypeError
};

#[cfg(test)]
mod tests;

pub fn type_check(mut resolved_ast: ResolvedAst) -> ResolvedAst {
    let mut type_checker = TypeChecker::new(resolved_ast.symtable, resolved_ast.error_mode);
    for statement in &mut resolved_ast.ast {
        check_statement(statement, &mut type_checker);
    }
    ResolvedAst { 
        ast: resolved_ast.ast, 
        symtable: type_checker.symtable, 
        error_mode: type_checker.error_mode
    }
}

struct TypeChecker {
    symtable: Symtable,
    error_stack: Vec<TypeCheckError>,
    error_mode: bool
}

impl TypeChecker {
    fn new (symtable: Symtable, error_mode: bool) -> Self {
        Self {
            error_stack: Vec::new(),
            symtable,
            error_mode
        }
    }

    fn push_error(&mut self, error: TypeCheckError) {
        self.error_stack.push(error);
        if !self.error_mode {
            self.error_mode = true;
        }
    }

    fn get_vartype(&self, id: usize) -> Option<TiroType> {
        self.symtable.variable_table[id].clone()
    }

    fn assign_vartype(&mut self, id: usize, vartype: TiroType) {
        self.symtable.variable_table[id] = Some(vartype);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TiroType {
    StringType
}

fn check_statement(statement: &mut Statement, type_checker: &mut TypeChecker) {
    match statement {
        Statement::Print {value} => check_print(value, type_checker),
        Statement::VariableAssignment {identifier, value} => check_variable_assignment(identifier, value, type_checker),
        _ => panic!("Unsupported Statement type")
    };
}

fn check_print(value: &mut Expression, type_checker: &mut TypeChecker) {
    if let Some(tiro_type) = check_expression(value, type_checker) {
        if tiro_type != TiroType::StringType {
            type_checker.push_error(TypeCheckError::MismatchedTypeError(
                    TypeError::PrintValueError(TiroType::StringType)));
        }
    }
}

fn check_variable_assignment(identifier: &mut Symbol, value: &mut Expression, type_checker: &mut TypeChecker) {
    if let Symbol::Id(id) = identifier {
        let variable_type = type_checker.get_vartype(*id);
        if let Some(value_type) = check_expression(value, type_checker) {
            match variable_type {
                Some(var_type) => if var_type != value_type {
                    type_checker.push_error(TypeCheckError::MismatchedTypeError(
                            TypeError::VariableAssignmentError(var_type, value_type)));
                },
                None => type_checker.assign_vartype(*id, value_type)
            }
        }
    } else { panic!("Unexpected unresolved id") }
}

fn check_expression(expression: &mut Expression, type_checker: &mut TypeChecker) -> Option<TiroType> {
    match expression {
        Expression::StringValue {..} => Some(TiroType::StringType),
        Expression::Variable {identifier} => check_variable(identifier, type_checker),
        _ => panic!("Unsupported expression type")
    }
}

fn check_variable(identifier: &mut Symbol, type_checker: &mut TypeChecker) -> Option<TiroType> {
    if let Symbol::Id(id) = identifier {
        let vartype = type_checker.get_vartype(*id);
        if vartype == None { panic!("Unexpected unintialized variable") }
        vartype
    } else { panic!("Unexpected unresolved identifier") }
}
