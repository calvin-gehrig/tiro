use std::collections::HashMap;
use std::mem;

use crate::common::{
    ResolvedAst,
    Symtable,
    Statement,
    Expression,
    Parameter,
    ParamType,
    Function,
    Symbol,
    TiroType
};

mod error;
use error::ReferenceError;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, PartialEq)]
enum Reference {
    Variable(usize),
    Function(usize)
}

#[derive(Default)]
struct Environment {
    current_environment: HashMap<String, Reference>,
    upper_environment: UpperEnv
}

impl Environment {
    fn push_ref(&mut self, name: String, reference: Reference) {
       self.current_environment.insert(name, reference);
    }

    fn get_id(&self, name: &String) -> Option<Reference> {
        let mut index = self.current_environment.get(name).cloned();
        if index.is_none() {
            if let UpperEnv::Env(upper_environment) = &self.upper_environment {
                index = upper_environment.get_id(name);
            }
        }
        index
    }

    fn open_scope(&mut self) {
        let upper_environment = mem::take(self);
        *self = Environment {
            current_environment: HashMap::new(),
            upper_environment: UpperEnv::Env(Box::new(upper_environment))
        }
    }

    fn end_scope(&mut self) {
        if let UpperEnv::Env(upper_env) = mem::take(&mut self.upper_environment) {
            *self = *upper_env;
        } else { panic!("Unexpected EOE") }
    }
}

#[derive(Default)]
enum UpperEnv {
    Env(Box<Environment>),
    #[default]
    EOE
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
                variable_table: Vec::new(),
                function_table: Vec::new()
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
            let index = self.symtable.variable_table.len() - 1;
            self.environment.push_ref(name, Reference::Variable(index));
            index
        } else { panic!("Unexpected resolved variable") }
    }

    fn register_function(&mut self, symbol: Symbol) -> usize {
        if let Symbol::Name(name) = symbol {
            let empty_function = Function {
                return_type: None,
                param_list: vec![]
            };
            self.symtable.function_table.push(empty_function);
            let index = self.symtable.function_table.len() - 1;
            self.environment.push_ref(name, Reference::Function(index));
            index
        } else { panic!("Unexpected resolved function") }
    }

    fn declare_function(&mut self,
        index: usize,
        return_type: Option<TiroType>,
        param_list: Vec<ParamType>
    ) {
        let maybe_function = self.symtable.function_table.get_mut(index);
        if let Some(function) = maybe_function {
            function.return_type = return_type;
            function.param_list = param_list;
        } else { panic!("Corrupted function index") }
}

    fn resolve_variable(&self, symbol: Symbol) -> Result<usize, ReferenceError> {
        if let Symbol::Name(name) = symbol {
            match self.environment.get_id(&name) {
                Some(reference) => if let Reference::Variable(index) = reference {
                    Ok(index)
                } else {
                    Err(ReferenceError::InvalidSymbolUseAsVariable(name))
                },
                None => Err(ReferenceError::UndefinedVariableName(name))
            }
        } else { panic!("Unexpected resolved variable") }
    }

    fn resolve_function(&self, symbol: Symbol) -> Result<usize, ReferenceError> {
        if let Symbol::Name(name) = symbol {
            match self.environment.get_id(&name) {
                Some(reference) => if let Reference::Function(index) = reference {
                    Ok(index)
                } else {
                    Err(ReferenceError::InvalidSymbolUseAsFunction(name))
                },
                None => Err(ReferenceError::UndefinedFunctionName(name))
            }
        } else { panic!("Unexpected resolved function") }
    }

    fn open_scope(&mut self) {
        self.environment.open_scope();
    }

    fn end_scope(&mut self) {
        self.environment.end_scope();
    }
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
            Ok(statement) => statement,
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

fn filter_error<T>(vector: Vec<Result<T, ReferenceError>>, resolver: &mut Resolver) -> Vec<T> {
    let mut error_stack = Vec::new();
    let vector = vector.into_iter().filter_map(|resolving_result| {
        match resolving_result {
            Ok(value) => Some(value),
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
    vector
}

fn resolve_statement(statement: Statement, resolver: &mut Resolver) -> Result<Option<Statement>, ReferenceError> {
    match statement {
        Statement::Print {value} => Ok(Some(Statement::Print {value: resolve_expression(value, resolver)?} )),
        Statement::VariableDeclaration {
            value,
            identifier,
            variable_type
        } => resolve_variable_declaration(
            value,
            identifier,
            variable_type,
            resolver),
        Statement::FunctionDeclaration {
            identifier,
            param_list,
            return_type,
            block
        } => resolve_function_declaration(
            identifier,
            param_list,
            return_type,
            block,
            resolver),
        Statement::Call {expression} => Ok(Some(Statement::Call {expression: resolve_expression(expression, resolver)?} )),
        Statement::ReturnStatement {return_value, function} => resolve_return(return_value, function, resolver),
        _ => panic!("Unsupported statement type: {:?}", statement)
    }
}

fn resolve_variable_declaration(value: Expression, identifier: Symbol, variable_type: Option<Symbol>, resolver: &mut Resolver) -> Result<Option<Statement>, ReferenceError> {
    let value = resolve_expression(value, resolver)?;
    let variable_type = resolver.resolve_type(variable_type)?;
    let id = resolver.declare_variable(identifier, variable_type);
    Ok(Some(Statement::VariableAssignment {
        identifier: Symbol::Id(id),
        value
    }))
}

fn resolve_function_declaration(
    identifier: Symbol,
    param_list: Vec<Parameter>,
    return_type: Option<Symbol>,
    block: Box<Vec<Statement>>,
    resolver: &mut Resolver) -> Result<Option<Statement>, ReferenceError> {
    let index = resolver.register_function(identifier);
    let return_type = resolver.resolve_type(return_type)?;

    resolver.open_scope();
    let param_list = param_list.into_iter().map(|parameter| {
        declare_parameter(parameter, resolver)
    }).collect();

    let block = resolve_block(*block, resolver);
    resolver.end_scope();

    let param_list = filter_error(param_list, resolver);
    resolver.declare_function(index, return_type, param_list);
    Ok(Some(Statement::FunctionDefinition {
        identifier: Symbol::Id(index),
        block: Box::new(block)
    }))
}

fn declare_parameter (parameter: Parameter, resolver: &mut Resolver) -> Result<ParamType, ReferenceError> {
    let param_type = resolver.resolve_type(Some(parameter.param_type))?;
    let id = resolver.declare_variable(parameter.identifier, param_type.clone());
    Ok(ParamType {
        identifier: Symbol::Id(id),
        param_type: param_type.expect("Unexpected untyped parameter")
    })
}

fn resolve_return(mut return_value: Option<Expression>, function: Symbol, resolver: &mut Resolver) -> Result<Option<Statement>, ReferenceError> {
    if let Some(value) = return_value {
        return_value = Some(resolve_expression(value, resolver)?);
    }
    let id = resolver.resolve_function(function)?;
    Ok(Some(Statement::ReturnStatement {
        return_value, 
        function: Symbol::Id(id)
    }))
}


fn resolve_expression(expression: Expression, resolver: &mut Resolver) -> Result<Expression, ReferenceError> {
    match expression {
        Expression::Variable {identifier} => resolve_variable(identifier, resolver),
        Expression::FunctionCall {
            identifier,
            argument_list
        } => resolve_function_call(identifier, argument_list, resolver),
        _ => Ok(expression)
    }
}

fn resolve_variable(identifier: Symbol, resolver: &mut Resolver) -> Result<Expression, ReferenceError> {
    let id = resolver.resolve_variable(identifier)?;
    Ok(Expression::Variable {identifier: Symbol::Id(id)})
}

fn resolve_function_call(identifier: Symbol, argument_list: Box<Vec<Expression>>, resolver: &mut Resolver) -> Result<Expression, ReferenceError> {
    let id = resolver.resolve_function(identifier)?;
    let argument_list = argument_list.into_iter().map(|argument| {
        resolve_expression(argument, resolver)
    }).collect();
    Ok(Expression::FunctionCall {
        identifier: Symbol::Id(id),
        argument_list: Box::new(filter_error(argument_list, resolver))
    })
}
