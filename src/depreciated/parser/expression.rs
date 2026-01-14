use crate::lexer::Token;
use super::IterToken;

pub fn parse_expression(token: Token, tokens: IterToken) -> (Expression, IterToken) {
    parse_equality(token, tokens)
}

#[derive(Debug)]
pub enum Expression {
    Binary {
        left_operand: Box<Expression>,
        right_operand: Box<Expression>,
        operator: Token
    },
    Id(String),
    Number(u32),
    StringNode(String)
}

fn parse_equality(left_token: Token, mut tokens: IterToken) -> (Expression, IterToken) {
    let (mut expression, returned_tokens) = parse_binary(left_token, tokens);
    tokens = returned_tokens;
    loop {
        match tokens.peek() {
            Some(token) => match token {
                Token::Whitespace => {
                    tokens.next();
                    continue
                },
                Token::EqualOperator | Token::NotEqualOperator => {
                    let operator = token.clone();
                    tokens.next();
                    let (right_expression, returned_tokens) = match tokens.next() {
                        Some(token) => parse_binary(token.clone(), tokens),
                        None => panic!("Unexpected end of program")
                    };
                    tokens = returned_tokens;
                    expression = Expression::Binary {
                        left_operand: Box::new(expression),
                        right_operand: Box::new(right_expression),
                        operator
                    }
                },
                _ => break (expression, tokens)
            },
            None => break (expression, tokens)
        }
    }
}

fn parse_binary(left_token: Token, mut tokens: IterToken) -> (Expression, IterToken) {
    let (mut expression, returned_tokens) = parse_value(left_token, tokens);
    tokens = returned_tokens;
    loop {
        match tokens.peek() {
            Some(token) => match token {
                Token::Whitespace => {
                    tokens.next();
                    continue
                },
                Token::AddOperator | Token::SubtractOperator | Token::MultiplyOperator | Token::DivideOperator => {
                    let operator = token.clone();
                    tokens.next();
                    let (right_expression, returned_tokens) = match tokens.next() {
                        Some(token) => parse_value(token.clone(), tokens),
                        None => panic!("Unexpected end of program")
                    };
                    tokens = returned_tokens;
                    expression = Expression::Binary {
                        left_operand: Box::new(expression),
                        right_operand: Box::new(right_expression),
                        operator
                    }
                },
                _ => break (expression, tokens)
            },
            None => break (expression, tokens)
        }
    }
}

fn parse_value(left_token: Token, mut tokens: IterToken) -> (Expression, IterToken) {
    let value = match left_token {
        Token::Id(identifier) => Expression::Id(identifier),
        Token::Number(number) => Expression::Number(number),
        Token::StringToken(string) => Expression::StringNode(string),
        _ => panic!("Unexpected token")
    };
    (value, tokens)
}
