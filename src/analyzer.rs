use crate::parser::{
    Statement,
    Expression
};

pub fn analyze (ast: AstNode) -> Vec<Vec<&mut Statement>> {
    let analyzer: Analyzer = Analyzer::new();
    for statement in main_block {
        match statement {
            Statement::Print { &value } => {
                analyze_print(value, analyzer);
                analyzer.push(statement);
            },
            Statement::Let { &identifier, &mut value } => {
                let index = analyze_let(identifier, value, analyzer);
                analyzer.push(Statement::Initialization { index, value };
            }
        }
    }
}

fn analyze_print(value: &Expression, analyzer: Analyzer) {
    let value_type = analyze_expression(value, analyzer);
    if (value_type != Type::StringType) {
        panic!("Unexpected type")
    }
}

fn analyze_let(identifier: &String, value: &Expression, analyzer: Analyzer) {
    let var_type = analyze_expression(var_type, analyzer);
    analyzer.add_variable(Variable::new(identifier, var_type, value))
}

fn analyze_expression(expression: Expression, 
