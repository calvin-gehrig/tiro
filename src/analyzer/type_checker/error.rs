use crate::common::TiroType;

pub enum TypeCheckError {
    MismatchedTypeError(TypeError),
}

pub enum TypeError {
    PrintValueError(TiroType),
    VariableAssignmentError(TiroType, TiroType)
}
