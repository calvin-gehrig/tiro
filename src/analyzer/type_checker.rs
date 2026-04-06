use crate::common::{
    ResolvedAst,
    Symtable,
    Statement,
    Expression,
    Function,
    LocalVariable,
    Type,
    OperationType
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
        self.error_mode = true;
    }

    fn get_vartype(&self, id: usize) -> Option<Type> {
        let variable = &self.symtable.variable_table[id];
        variable.vartype.clone()
    }

    fn get_variable(&self, id: usize) -> LocalVariable {
        self.symtable.variable_table[id].clone()
    }

    fn get_function(&self, id: usize) -> Function {
        self.symtable.function_table[id].clone()
    }

    fn assign_vartype(&mut self, id: usize, vartype: Type) {
        self.symtable.variable_table[id].vartype = Some(vartype);
    }
}

fn check_statement(statement: &mut Statement, type_checker: &mut TypeChecker) {
    match statement {
        Statement::Print {value} => check_print(value, type_checker),
        Statement::VariableAssignment {identifier, value} => check_variable_assignment(*identifier, value, type_checker),
        Statement::FunctionDefinition {block,..} => check_function_definition(block, type_checker),
        Statement::Call {expression} => {
            check_expression(expression, type_checker);
        },
        Statement::ResolvedReturn {return_value, function} => check_return(return_value, *function, type_checker),
        _ => panic!("Unsupported Statement type {:?}", statement)
    };
}

fn check_print(value: &mut Expression, type_checker: &mut TypeChecker) {
    let tiro_type = check_expression(value, type_checker);
    if tiro_type != Type::StringType {
        type_checker.push_error(TypeCheckError::MismatchedTypeError(
                TypeError::PrintValueError(tiro_type)));
    }
}

fn check_variable_assignment(id: usize, value: &mut Expression, type_checker: &mut TypeChecker) {
    let variable = type_checker.get_variable(id);
    let value_type = check_expression(value, type_checker);
    match variable.vartype {
        Some(variable_type) => if variable_type != value_type {
            type_checker.push_error(TypeCheckError::MismatchedTypeError(
                    TypeError::VariableAssignmentError(
                        variable.identifier,
                        variable_type, 
                        value_type)));
        },
        None => type_checker.assign_vartype(id, value_type)
    }
}

fn check_function_definition(block: &mut Vec<Statement>, type_checker: &mut TypeChecker) {
    for statement in block {
        check_statement(statement, type_checker);
    }
}

fn check_return(maybe_value: &mut Option<Expression>, id: usize, type_checker: &mut TypeChecker) {
    let function = type_checker.get_function(id);
    if let Some(return_value) = maybe_value {
        let value_type = check_expression(return_value, type_checker);
        if let Some(return_type) = function.return_type {
            if return_type != value_type {
                type_checker.push_error(TypeCheckError::MismatchedTypeError(
                        TypeError::ReturnedValueError(
                            function.identifier,
                            return_type,
                            value_type)));
            }
        } else if !value_type.is_null() {
            type_checker.push_error(TypeCheckError::ReturnError(
                    ReturnErr::ValueReturnedOnNull(
                        function.identifier
                    )));
        }
    } else {
        if function.return_type.is_some() {
            type_checker.push_error(TypeCheckError::ReturnError(
                    ReturnErr::NullReturnedOnValue(
                        function.identifier
                    )));
        }
    }
}

fn check_expression(expression: &mut Expression, type_checker: &mut TypeChecker) -> Type {
    match expression {
        Expression::StringValue {..} => Type::StringType,
        Expression::Number {..} => Type::Integer,
        Expression::LocalVar {id, ..} => check_variable(*id, type_checker),
        Expression::ResolvedFunctionCall {id, argument_list} => check_function_call(*id, argument_list, type_checker),
        Expression::BinaryOperation {lhs, rhs, op_type} => check_binary(
            &mut *lhs,
            &mut *rhs,
        op_type, type_checker),
        _ => panic!("Unsupported expression type")
    }
}

fn check_variable(id: usize, type_checker: &mut TypeChecker) -> Type {
        let maybe_vartype = type_checker.get_vartype(id);
        match maybe_vartype {
            Some(vartype) => vartype,
            None => { panic!("Unexpected unintialized variable") }
        }
}

fn check_function_call(id: usize, argument_list: &mut Box<Vec<Expression>>, type_checker: &mut TypeChecker) -> Type {
    let function = type_checker.get_function(id);
    if argument_list.len() != function.param_list.len() {
        type_checker.push_error(TypeCheckError::ArityError(
                function.identifier,
                function.param_list.len(),
                argument_list.len()));
    } else {
        let name = function.identifier.clone();
        argument_list.iter_mut()
            .zip(function.param_list.iter())
            .for_each(|(argument, parameter)| {
                let argument_type = check_expression(argument, type_checker);
                let param_type = &parameter.param_type;
                if argument_type != *param_type {
                    type_checker.push_error(TypeCheckError::MismatchedTypeError(TypeError::ParameterArgumentError(
                                name.clone(),
                                parameter.identifier.clone(),
                                param_type.clone(),
                                argument_type
                    )));
                }
        });
    }

    match &function.return_type {
        Some(return_type) => return_type.clone(),
        None => Type::null()
    }
}

fn check_binary (lhs: &mut Expression, rhs: &mut Expression, op_type: &mut OperationType, type_checker: &mut TypeChecker) -> Type {
    let expected_type = match op_type {
        OperationType::Add 
        | OperationType::Sub
        | OperationType::Mul
        | OperationType::Div
        | OperationType::Pow => Type::Integer,
        OperationType::Cat => Type::StringType,
    };
    let lhs = check_expression(lhs, type_checker);
    let rhs = check_expression(rhs, type_checker);
    if lhs != expected_type || rhs != expected_type {
        type_checker.push_error(
            TypeCheckError::MismatchedTypeError(
                TypeError::BinaryOperandError(
                    op_type.clone(),
                    expected_type.clone(),
                    lhs,
                    rhs
                )));
    }
    expected_type
}
