use crate::common::{
    Type,
    OperationType
};

#[derive(Debug, Clone, PartialEq)]
pub enum TypeCheckError {
    MismatchedTypeError(TypeError),
    ReturnError(ReturnErr),
    ArityError(String, usize, usize)
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReturnErr {
    ValueReturnedOnNull(String),
    NullReturnedOnValue(String)
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeError {
    PrintValueError(Type),
    VariableAssignmentError(String, Type, Type),
    ReturnedValueError(String, Type, Type),
    ParameterArgumentError(String, String, Type, Type),
    BinaryOperandError(OperationType, Type, Type, Type),
    ConditionError(Type)
}
