use crate::common::{
    ResolvedAst,
    Symtable,
    Statement,
    Expression,
    Symbol,
    Function,
    TiroType
};

mod error;
use error::{
    TypeCheckError,
    TypeError,
    ReturnErr
};

#[cfg(test)]
mod tests;

pub fn type_check(mut resolved_ast: ResolvedAst) -> ResolvedAst {
    let mut type_checker = TypeChecker::new(resolved_ast.symtable, resolved_ast.error_mode);
    for statement in &mut resolved_ast.ast {
        check_statement(statement, &mut type_checker);
    }
    if type_checker.error_stack.len() > 0 {
        for error in &type_checker.error_stack {
            println!("{:?}", error);
        }
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

    fn get_vartype(&self, symbol: &Symbol) -> Option<TiroType> {
        if let Symbol::Id(id) = symbol {
            self.symtable.variable_table[*id].clone()
        } else { panic!("Unexpected unresolved identifier") }
    }

    fn get_function(&self, symbol: &Symbol) -> Function {
        if let Symbol::Id(id) = symbol {
            self.symtable.function_table[*id].clone()
        } else { panic!("Unexpected unresolved identifier") }
    }

    fn assign_vartype(&mut self, symbol: &Symbol, vartype: TiroType) {
        if let Symbol::Id(id) = symbol {
            self.symtable.variable_table[*id] = Some(vartype);
        } else { panic!("Unexpected unresolved identifier") }
    }
}

fn check_statement(statement: &mut Statement, type_checker: &mut TypeChecker) {
    match statement {
        Statement::Print {value} => check_print(value, type_checker),
        Statement::VariableAssignment {identifier, value} => check_variable_assignment(identifier, value, type_checker),
        Statement::FunctionDefinition {block,..} => check_function_definition(block, type_checker),
        Statement::Call {expression} => {
            check_expression(expression, type_checker);
        },
        Statement::ReturnStatement {return_value, function} => check_return(return_value, function, type_checker),
        _ => panic!("Unsupported Statement type {:?}", statement)
    };
}

fn check_print(value: &mut Expression, type_checker: &mut TypeChecker) {
    let tiro_type = check_expression(value, type_checker);
    if tiro_type != TiroType::StringType {
        type_checker.push_error(TypeCheckError::MismatchedTypeError(
                TypeError::PrintValueError(TiroType::StringType)));
    }
}

fn check_variable_assignment(identifier: &mut Symbol, value: &mut Expression, type_checker: &mut TypeChecker) {
    let variable_type = type_checker.get_vartype(identifier);
    let value_type = check_expression(value, type_checker);
    match variable_type {
        Some(var_type) => if var_type != value_type {
            type_checker.push_error(TypeCheckError::MismatchedTypeError(
                    TypeError::VariableAssignmentError(var_type, value_type)));
        },
        None => type_checker.assign_vartype(identifier, value_type)
    }
}

fn check_function_definition(block: &mut Vec<Statement>, type_checker: &mut TypeChecker) {
    for statement in block {
        check_statement(statement, type_checker);
    }
}

fn check_return(maybe_value: &mut Option<Expression>, function: &mut Symbol, type_checker: &mut TypeChecker) {
    let function = type_checker.get_function(function);
    if let Some(return_value) = maybe_value {
        let value_type = check_expression(return_value, type_checker);
        if let Some(return_type) = function.return_type {
            if return_type != value_type {
                type_checker.push_error(TypeCheckError::MismatchedTypeError(
                        TypeError::ReturnedValueError(return_type, value_type)));
            }
        } else if !value_type.is_null() {
            type_checker.push_error(TypeCheckError::ReturnError(
                    ReturnErr::ValueReturnedOnNull));
        }
    } else {
        if function.return_type.is_some() {
            type_checker.push_error(TypeCheckError::ReturnError(
                    ReturnErr::NullReturnedOnValue));
        }
    }
}

fn check_expression(expression: &mut Expression, type_checker: &mut TypeChecker) -> TiroType {
    match expression {
        Expression::StringValue {..} => TiroType::StringType,
        Expression::Variable {identifier} => check_variable(identifier, type_checker),
        Expression::FunctionCall {identifier, argument_list} => check_function_call(identifier, argument_list, type_checker),
        _ => panic!("Unsupported expression type")
    }
}

fn check_variable(identifier: &mut Symbol, type_checker: &mut TypeChecker) -> TiroType {
        let maybe_vartype = type_checker.get_vartype(identifier);
        match maybe_vartype {
            Some(vartype) => vartype,
            None => { panic!("Unexpected unintialized variable") }
        }
}

fn check_function_call(identifier: &mut Symbol, argument_list: &mut Box<Vec<Expression>>, type_checker: &mut TypeChecker) -> TiroType {
    let function = type_checker.get_function(identifier);
    if argument_list.len() != function.param_list.len() {
        type_checker.push_error(TypeCheckError::ArityError(
                function.param_list.len(),
                argument_list.len()));
    } else {
        argument_list.iter_mut()
            .zip(function.param_list.iter())
            .for_each(|(argument, parameter)| {
                let argument_type = check_expression(argument, type_checker);
                let param_type = &parameter.param_type;
                if argument_type != *param_type {
                    type_checker.push_error(TypeCheckError::MismatchedTypeError(TypeError::ParameterArgumentError(
                                argument_type,
                                param_type.clone()
                    )));
                }
        });
    }

    match &function.return_type {
        Some(return_type) => return_type.clone(),
        None => TiroType::null()
    }
}
