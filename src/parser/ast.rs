#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Print {value: Expression}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    StringValue {value: String}
}
