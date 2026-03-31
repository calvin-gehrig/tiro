use crate::common::TiroType;

#[derive(Debug, Clone, PartialEq)]
pub enum TypeCheckError {
    MismatchedTypeError(TypeError),
    ReturnError(ReturnErr),
    ArityError(usize, usize)
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReturnErr {
    ValueReturnedOnNull,
    NullReturnedOnValue
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeError {
    PrintValueError(TiroType),
    VariableAssignmentError(TiroType, TiroType),
    ReturnedValueError(TiroType, TiroType),
    ParameterArgumentError(TiroType, TiroType)
}
