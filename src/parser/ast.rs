#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Print {value: Expression},
    VariableDeclaration {
        value: Expression,
        identifier: Symbol,
        variable_type: Option<Symbol>
    },
    VariableAssignment {
        value: Expression,
        identifier: Symbol
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    StringValue {value: String},
    Variable {identifier: Symbol}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    Name(String),
    Id(usize)
}
