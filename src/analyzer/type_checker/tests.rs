use crate::parser::ast::{
    Statement,
    Expression
};

use super::type_check;

#[test]
fn print () {
    let print_statement = vec![
        Statement::Print {value:
            Expression::StringValue {value: "a".to_string()}
    }];
    assert_eq!(type_check(print_statement.clone()), print_statement);
}
