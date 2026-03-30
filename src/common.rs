#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedAst {
    pub ast: Vec<Statement>,
    pub symtable: Symtable,
    pub error_mode: bool
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyzedAst {
    pub symtable: Symtable,
    pub ast: Vec<Statement>
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub return_type: Option<TiroType>,
    pub param_list: Vec<ParamType>
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symtable {
    pub variable_table: Vec<Option<TiroType>>,
    pub function_table: Vec<Function>
}

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
    },
    FunctionDeclaration {
        identifier: Symbol,
        param_list: Vec<Parameter>,
        return_type: Option<Symbol>,
        block: Box<Vec<Statement>>
    },
    ReturnStatement {
        return_value: Option<Expression>
    },
    Call {
        expression: Expression
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub identifier: Symbol,
    pub param_type: Symbol
}


#[derive(Debug, Clone, PartialEq)]
pub struct ParamType {
    pub identifier: Symbol,
    pub param_type: TiroType
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    StringValue {value: String},
    Variable {identifier: Symbol},
    FunctionCall {
        identifier: Symbol,
        argument_list: Box<Vec<Expression>>
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    Name(String),
    Id(usize)
}

#[derive(Debug, Clone, PartialEq)]
pub enum TiroType {
    StringType
}
