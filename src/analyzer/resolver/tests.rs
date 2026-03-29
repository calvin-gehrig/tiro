use crate::parser::ast::{
    Statement,
    Expression,
    Symbol
};

use super::{
    ResolvedAst,
    Symtable,
    Resolver,
    resolve,
    resolve_block,
    error::ReferenceError
};

use crate::analyzer::type_checker::TiroType;

fn resolve_error(ast: Vec<Statement>) -> Vec<ReferenceError> {
    let mut resolver = Resolver::new();
    resolve_block(ast, &mut resolver);
    resolver.error_stack
}

#[test]
fn variable() {
    assert_eq!(resolve(vec![
        Statement::VariableDeclaration {
            value: Expression::StringValue {value:"a".to_string()},
            identifier: Symbol::Name("a".to_string()),
            variable_type: Some(Symbol::Name("cat".to_string()))
        },
        Statement::Print {
            value: Expression::Variable {
                identifier:Symbol::Name("a".to_string())
            }
        }
    ]), ResolvedAst {
        ast: vec![
            Statement::VariableAssignment {
                value: Expression::StringValue {value:"a".to_string()},
                identifier: Symbol::Id(0)
            },
            Statement::Print {
                value: Expression::Variable {
                    identifier:Symbol::Id(0)
                }
            }
        ],
        symtable: Symtable {
            variable_table: vec![Some(TiroType::StringType)]
        },
        error_mode: false
    });
}

#[test]
fn variable_error() {
    assert_eq!(resolve_error(vec![
            Statement::Print {
                value: Expression::Variable {
                    identifier:Symbol::Name("a".to_string())
                }
            }
    ]), vec![ReferenceError::UndefinedVariableName("a".to_string())]);
}

#[test]
fn type_error() {
    assert_eq!(resolve_error(vec![
            Statement::VariableDeclaration {
                value: Expression::StringValue {value:"a".to_string()},
                identifier: Symbol::Name("a".to_string()),
                variable_type: Some(Symbol::Name("ouh".to_string()))
            }
    ]), vec![ReferenceError::UndefinedTypeName("ouh".to_string())]);
}
