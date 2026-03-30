use crate::common::TiroType;

pub enum TypeCheckError {
    MismatchedTypeError(TypeError),
    ReturnError(ReturnErr),
    ArityError(usize, usize)
}

pub enum ReturnErr {
    ValueReturnedOnNull,
    NullReturnedOnValue
}

pub enum TypeError {
    PrintValueError(TiroType),
    VariableAssignmentError(TiroType, TiroType),
    ReturnedValueError(TiroType, TiroType),
    ParameterArgumentError(TiroType, TiroType)
}
