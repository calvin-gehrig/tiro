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
    pub return_type: Option<Type>,
    pub param_list: Vec<ParamType>,
    pub identifier: String
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalVariable {
    pub vartype: Option<Type>,
    pub identifier: String,
    pub index: usize
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symtable {
    pub variable_table: Vec<LocalVariable>,
    pub function_table: Vec<Function>
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Print {value: Expression},
    VariableDeclaration {
        value: Expression,
        identifier: String,
        variable_type: Option<String>
    },
    VariableAssignment {
        value: Expression,
        identifier: usize
    },
    FunctionDeclaration {
        identifier: String,
        param_list: Vec<Parameter>,
        return_type: Option<String>,
        block: Box<Vec<Statement>>
    },
    FunctionDefinition {
        identifier: usize,
        block: Box<Vec<Statement>>
    },
    ReturnStatement {
        return_value: Option<Expression>,
        function: String
    },
    ResolvedReturn {
        return_value: Option<Expression>,
        function: usize
    },
    Call {
        expression: Expression
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub identifier: String,
    pub param_type: String
}


#[derive(Debug, Clone, PartialEq)]
pub struct ParamType {
    pub identifier: String,
    pub param_type: Type
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    StringValue {value: String},
    Number {value: u32},
    Boolean {value: bool},
    Variable {identifier: String},
    LocalVar {id: usize, depth: usize},
    BinaryOperation {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
        op_type: OperationType
    },
    Cast {
        operand: Box<Expression>,
        output_type: String
    },
    ResolvedCast {
        operand: Box<Expression>,
        output_type: Type
    },
    AnalyzedCast {
        operand: Box<Expression>,
        output_type: Type,
        input_type: Type
    },
    FunctionCall {
        identifier: String,
        argument_list: Box<Vec<Expression>>
    },
    ResolvedFunctionCall {
        id: usize,
        argument_list: Box<Vec<Expression>>
    }
}

impl Expression {
    pub fn binary(lhs: Expression, rhs: Expression, op_type: OperationType) -> Self {
        Self::BinaryOperation {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            op_type
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    StringType,
    Integer,
    Boolean,
    Null(Nil)
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperationType {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Cat
}

impl Type {
    pub fn null() -> Self {
        Self::Null(Nil {})
    }
    pub fn is_null(&self) -> bool {
        match self {
            Self::Null(_) => true,
            _ => false
        }
    }
}

#[derive(Debug, Clone)]
pub struct Nil {}

impl PartialEq for Nil {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
