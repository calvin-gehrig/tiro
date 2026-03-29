#[derive(Debug, Clone, PartialEq)]
pub enum ReferenceError {
    UndefinedTypeName(String),
    UndefinedVariableName(String)
}
