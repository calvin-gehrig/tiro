use std::collections::HashMap;

use crate::parser::ast::{
    Statement,
    Expression,
    Symbol
};

use super::type_checker::TiroType;

mod error;
use error::ReferenceError;

#[cfg(test)]
mod tests;

struct Environment {
    current_environment: HashMap<String, usize>,
    upper_environment: UpperEnv
}

impl Environment {
    fn push_ref(&mut self, name: String, index: usize) -> usize {
       self.current_environment.insert(name, index);
       index
    }

    fn get_id(&self, name: &String) -> Option<usize> {
        let mut index = self.current_environment.get(name).copied();
        if index.is_none() {
            if let UpperEnv::Env(upper_environment) = &self.upper_environment {
                index = upper_environment.get_id(name);
            }
        }
        index
    }
}

enum UpperEnv {
    Env(Box<Environment>),
    EOE
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symtable {
    pub variable_table: Vec<Option<TiroType>>
}

struct Resolver {
    environment: Environment,
    symtable: Symtable,
    error_stack: Vec<ReferenceError>,
    error_mode: bool
}

impl Resolver {
    fn new() -> Self {
        Self {
            environment: Environment {
                current_environment: HashMap::new(),
                upper_environment: UpperEnv::EOE
            },
            symtable: Symtable {
                variable_table: Vec::new()
            },
            error_stack: Vec::new(),
            error_mode: false
        }
    }

    fn resolve_type(&self, maybe_type_symbol: Option<Symbol>) -> Result<Option<TiroType>, ReferenceError> {
        match maybe_type_symbol {
            Some(type_symbol) => if let Symbol::Name(type_name) = type_symbol {
                    match type_name.as_str() {
                        "cat" => Ok(Some(TiroType::StringType)),
                        _ => Err(ReferenceError::UndefinedTypeName(type_name))
                    }
                } else { panic!("Unexpected resolved type") },
            None => Ok(None)
        }
    }

    fn declare_variable(&mut self, symbol: Symbol, variable_type: Option<TiroType>) -> usize {
        if let Symbol::Name(name) = symbol {
            self.symtable.variable_table.push(variable_type);
            self.environment.push_ref(name, self.symtable.variable_table.len() - 1)
        } else { panic!("Unexpected resolved variable") }
    }

    fn resolve_variable(&self, symbol: Symbol) -> Result<usize, ReferenceError> {
        if let Symbol::Name(name) = symbol {
            match self.environment.get_id(&name) {
                Some(index) => Ok(index),
                None => Err(ReferenceError::UndefinedVariableName(name))
            }
        } else { panic!("Unexpected resolved variable") }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedAst {
    pub ast: Vec<Statement>,
    pub symtable: Symtable,
    pub error_mode: bool
}

pub fn resolve(ast: Vec<Statement>) -> ResolvedAst {
    let mut resolver = Resolver::new();
    let ast = resolve_block(ast, &mut resolver);
    if resolver.error_mode {
        println!("Resolving error:");
        for error in &resolver.error_stack {
            println!("{:?}", error);
        }
    }
    ResolvedAst { ast, symtable: resolver.symtable, error_mode: resolver.error_mode }
}

fn resolve_block(block: Vec<Statement>, resolver: &mut Resolver) -> Vec<Statement> {
    let mut error_stack = Vec::new();
    let resolved_block = block.into_iter().map(|statement| {
        resolve_statement(statement, resolver)
    }).filter_map(|resolving_result| {
        match resolving_result {
            Ok(statement) => Some(statement),
            Err(error) => {
                error_stack.push(error);
                None
            }
        }
    }).collect();
    if error_stack.len() > 0 {
        resolver.error_stack.append(&mut error_stack);
        resolver.error_mode = true;
    }
    resolved_block
}

fn resolve_statement(statement: Statement, resolver: &mut Resolver) -> Result<Statement, ReferenceError> {
    match statement {
        Statement::Print {value} => Ok(Statement::Print {value: resolve_expression(value, resolver)?}),
        Statement::VariableDeclaration {value, identifier, variable_type} => resolve_variable_declaration(value, identifier, variable_type, resolver),
        _ => panic!("Unsupported statement type")
    }
}

fn resolve_variable_declaration(value: Expression, identifier: Symbol, variable_type: Option<Symbol>, resolver: &mut Resolver) -> Result<Statement, ReferenceError> {
    let value = resolve_expression(value, resolver)?;
    let variable_type = resolver.resolve_type(variable_type)?;
    let id = resolver.declare_variable(identifier, variable_type);
    Ok(Statement::VariableAssignment {
        identifier: Symbol::Id(id),
        value
    })
}

fn resolve_expression(expression: Expression, resolver: &mut Resolver) -> Result<Expression, ReferenceError> {
    match expression {
        Expression::Variable {identifier} => resolve_variable(identifier, resolver),
        _ => Ok(expression)
    }
}

fn resolve_variable(identifier: Symbol, resolver: &mut Resolver) -> Result<Expression, ReferenceError> {
    let id = resolver.resolve_variable(identifier)?;
    Ok(Expression::Variable {identifier: Symbol::Id(id)})
}
