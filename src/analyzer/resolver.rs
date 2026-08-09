use std::collections::HashMap;
use std::mem;

use crate::common::{
    ResolvedAst,
    Symtable,
    Statement,
    Expression,
    Parameter,
    ParamType,
    OperationType,
    If,
    Function,
    LocalVariable,
    Type
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

#[derive(Default, Debug)]
struct Environment {
    current_environment: HashMap<String, Reference>,
    local_depth: usize,
    local_count: usize,
    upper_environment: UpperEnv
}

impl Environment {
    fn push_ref(&mut self, name: String, reference: Reference) {
        if let &Reference::Variable(_) = &reference {
            self.local_count += 1;
        }
        self.current_environment.insert(name, reference);
    }

    fn get_id(&self, name: &String, depth: usize) -> Option<(Reference, usize)> {
        let maybe_index = self.current_environment.get(name).cloned();
        match maybe_index {
            None => if let UpperEnv::Env(upper_environment) = &self.upper_environment {
                upper_environment.get_id(name, depth + 1)
                } else { None },
            Some(index) => Some((index, depth))
        }
    }

    fn open_scope(&mut self, is_function: bool) {
        let upper_environment = mem::take(self);
        *self = Environment {
            current_environment: HashMap::new(),
            local_depth: if is_function { 0 } else {
                upper_environment.local_depth + 1 },
            local_count: 0,
            upper_environment: UpperEnv::Env(Box::new(upper_environment))
        }
    }

    fn end_scope(&mut self) {
        if let UpperEnv::Env(upper_env) = mem::take(&mut self.upper_environment) {
            *self = *upper_env;
        } else { panic!("Unexpected EOE") }
    }
}

#[derive(Default, Debug)]
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
                local_depth: 0,
                local_count: 0,
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

    fn resolve_type(&self, maybe_type_name: Option<String>) -> Result<Option<Type>, ReferenceError> {
        match maybe_type_name {
            Some(type_name) => {
                    match type_name.as_str() {
                        "cat" => Ok(Some(Type::StringType)),
                        "num" => Ok(Some(Type::Integer)),
                        "bin" => Ok(Some(Type::Boolean)),
                        _ => Err(ReferenceError::UndefinedTypeName(type_name))
                    }
            },
            None => Ok(None)
        }
    }

    fn declare_variable(&mut self, name: String, variable_type: Option<Type>) -> usize {
        let variable = LocalVariable {
            vartype: variable_type,
            identifier: name.clone(),
            index: self.environment.local_count
        };
        self.symtable.variable_table.push(variable);
        let index = self.symtable.variable_table.len() - 1;
        self.environment.push_ref(name, Reference::Variable(index));
        index
    }

    fn register_function(&mut self, name: String) {
        let empty_function = Function {
            identifier: name.clone(),
            return_type: None,
            param_list: vec![]
        };
        self.symtable.function_table.push(empty_function);
        self.environment.push_ref(name, Reference::Function(self.symtable.function_table.len() - 1));
    }

    fn declare_function(&mut self,
        index: usize,
        return_type: Option<Type>,
        param_list: Vec<ParamType>
    ) {
        let maybe_function = self.symtable.function_table.get_mut(index);
        if let Some(function) = maybe_function {
            function.return_type = return_type;
            function.param_list = param_list;
        } else { panic!("Corrupted function index") }
}

    fn resolve_variable(&self, name: String) -> Result<(usize, usize), ReferenceError> {
        match self.environment.get_id(&name, 0) {
            Some((reference, depth)) => if let Reference::Variable(index) = reference {
                Ok((index, depth))
            } else {
                Err(ReferenceError::InvalidSymbolUseAsVariable(name))
            },
            None => Err(ReferenceError::UndefinedVariableName(name))
        }
    }

    fn resolve_function(&self, name: String) -> Result<usize, ReferenceError> {
        match self.environment.get_id(&name, 0) {
            Some((reference, _)) => if let Reference::Function(index) = reference {
                Ok(index)
            } else {
                Err(ReferenceError::InvalidSymbolUseAsFunction(name))
            },
            None => Err(ReferenceError::UndefinedFunctionName(name))
        }
    }

    fn open_scope(&mut self, is_function: bool) {
        self.environment.open_scope(is_function);
    }

    fn end_scope(&mut self) {
        self.environment.end_scope();
    }
}

pub fn resolve(ast: Vec<Statement>) -> ResolvedAst {
    let mut resolver = Resolver::new();
    register_global(&ast, &mut resolver);
    let ast = resolve_block(ast, &mut resolver);
    if resolver.error_mode {
        println!("Resolving error:");
        for error in &resolver.error_stack {
            println!("{:?}", error);
        }
    }
    ResolvedAst { ast, symtable: resolver.symtable, error_mode: resolver.error_mode }
}

fn register_global(ast: &Vec<Statement>, resolver: &mut Resolver) {
    for statement in ast {
        match statement {
            Statement::FunctionDeclaration {identifier,..} => resolver.register_function(identifier.clone()),
            _ => ()
        }
    }
}

fn resolve_scoped_block(block: Vec<Statement>, resolver: &mut Resolver) -> Vec<Statement> {
    resolver.open_scope(false);
    let block = resolve_block(block, resolver);
    resolver.end_scope();
    block
}

fn resolve_block(block: Vec<Statement>, resolver: &mut Resolver) -> Vec<Statement> {
    let resolved_block = block.into_iter().map(|statement| {
        resolve_statement(statement, resolver)
    }).collect();
    filter_error(resolved_block, resolver)
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

fn resolve_statement(statement: Statement, resolver: &mut Resolver) -> Result<Statement, ReferenceError> {
    match statement {
        Statement::Print {value} => Ok(Statement::Print {value: resolve_expression(value, resolver)?} ),
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
        Statement::Call {expression} => Ok(Statement::Call {expression: resolve_expression(expression, resolver)?} ),
        Statement::ReturnStatement {return_value, function} => resolve_return(return_value, function, resolver),
        Statement::IfElse {
            condition,
            if_block,
            else_block,
            elif_list
        } => resolve_if_else(condition, *if_block, *else_block, elif_list, resolver),
        _ => panic!("Unsupported statement type: {:?}", statement)
    }
}

fn resolve_variable_declaration(value: Expression, identifier: String, variable_type: Option<String>, resolver: &mut Resolver) -> Result<Statement, ReferenceError> {
    let value = resolve_expression(value, resolver)?;
    let variable_type = resolver.resolve_type(variable_type)?;
    let id = resolver.declare_variable(identifier, variable_type);
    Ok(Statement::VariableAssignment {
        identifier: id,
        value
    })
}

fn resolve_function_declaration(
    identifier: String,
    param_list: Vec<Parameter>,
    return_type: Option<String>,
    block: Box<Vec<Statement>>,
    resolver: &mut Resolver) -> Result<Statement, ReferenceError> {
    let index = resolver.resolve_function(identifier)?;
    let return_type = resolver.resolve_type(return_type)?;

    resolver.open_scope(true);
    let param_list = param_list.into_iter().map(|parameter| {
        declare_parameter(parameter, resolver)
    }).collect();

    let block = resolve_block(*block, resolver);
    resolver.end_scope();

    let param_list = filter_error(param_list, resolver);
    resolver.declare_function(index, return_type, param_list);
    Ok(Statement::FunctionDefinition {
        identifier: index,
        block: Box::new(block)
    })
}

fn declare_parameter (parameter: Parameter, resolver: &mut Resolver) -> Result<ParamType, ReferenceError> {
    let param_type = resolver.resolve_type(Some(parameter.param_type))?;
    resolver.declare_variable(parameter.identifier.clone(), param_type.clone());
    Ok(ParamType {
        identifier: parameter.identifier,
        param_type: param_type.expect("Unexpected untyped parameter")
    })
}

fn resolve_return(mut return_value: Option<Expression>, function: String, resolver: &mut Resolver) -> Result<Statement, ReferenceError> {
    if let Some(value) = return_value {
        return_value = Some(resolve_expression(value, resolver)?);
    }
    let id = resolver.resolve_function(function)?;
    Ok(Statement::ResolvedReturn {
        return_value, 
        function: id
    })
}

fn resolve_if_else(condition: Expression, if_block: Vec<Statement>, mut maybe_else_block: Option<Vec<Statement>>, elif_list: Vec<If>, resolver: &mut Resolver) -> Result<Statement, ReferenceError> {
    let condition = resolve_expression(condition, resolver)?;
    let if_block = resolve_scoped_block(if_block, resolver);

    if let Some(mut else_block) = maybe_else_block.take() {
        else_block = resolve_scoped_block(else_block, resolver);
        maybe_else_block = Some(else_block);
    } else {
        maybe_else_block = None;
    };

    let mut resolved_elif_list = Vec::new();
    for elif in elif_list {
        let condition = resolve_expression(elif.condition, resolver)?;
        let block = resolve_scoped_block(*elif.block, resolver);
        resolved_elif_list.push(If {condition, block: Box::new(block)});
    }
    Ok(Statement::IfElse {
        condition,
        if_block: Box::new(if_block),
        else_block: Box::new(maybe_else_block),
        elif_list: resolved_elif_list
    })
}


fn resolve_expression(expression: Expression, resolver: &mut Resolver) -> Result<Expression, ReferenceError> {
    match expression {
        Expression::Variable {identifier} => resolve_variable(identifier, resolver),
        Expression::FunctionCall {
            identifier,
            argument_list
        } => resolve_function_call(identifier, argument_list, resolver),
        Expression::BinaryOperation {lhs, rhs, op_type} => resolve_binary(*lhs, *rhs, op_type, resolver),
        Expression::Cast {operand, output_type} => resolve_cast(*operand, output_type, resolver),
        _ => Ok(expression)
    }
}

fn resolve_variable(identifier: String, resolver: &mut Resolver) -> Result<Expression, ReferenceError> {
    let (id, depth) = resolver.resolve_variable(identifier)?;
    Ok(Expression::LocalVar {
        id,
        depth
    })
}

fn resolve_function_call(identifier: String, argument_list: Box<Vec<Expression>>, resolver: &mut Resolver) -> Result<Expression, ReferenceError> {
    let id = resolver.resolve_function(identifier)?;
    let argument_list = argument_list.into_iter().map(|argument| {
        resolve_expression(argument, resolver)
    }).collect();
    Ok(Expression::ResolvedFunctionCall {
        id: id,
        argument_list: Box::new(filter_error(argument_list, resolver))
    })
}

fn resolve_binary(lhs: Expression, rhs: Expression, op_type: OperationType, resolver: &mut Resolver) -> Result<Expression, ReferenceError> {
    let lhs = Box::new(resolve_expression(lhs, resolver)?);
    let rhs = Box::new(resolve_expression(rhs, resolver)?);
    Ok(Expression::BinaryOperation {lhs, rhs, op_type})
}

fn resolve_cast(operand: Expression, output_type: String, resolver: &mut Resolver) -> Result<Expression, ReferenceError> {
    let output_type = resolver.resolve_type(Some(output_type))?
        .expect("Unexpected none type");
    let operand = Box::new(
        resolve_expression(operand, resolver)?);
    Ok(Expression::ResolvedCast {operand, output_type})
}
