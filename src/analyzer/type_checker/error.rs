use super::TiroType;

pub enum TypeCheckError {
    MismatchedTypeError(TypeError),
}

pub enum TypeError {
    PrintValueError(TiroType)
}
