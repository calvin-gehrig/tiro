use crate::lexer::Token;
use super::{
    IterToken,
    expression::{
        parse_expression,
        Expression
    }
};

pub fn parse_block(mut tokens: IterToken) -> (Vec<Statement>, IterToken) {
    let mut block: Vec<Statement> = Vec::new();
    loop { 
        match tokens.next() {
            Some(token) => {
                let (statement, returned_tokens) = match token {
                    Token::Print => parse_print(tokens),
                    Token::Let => parse_let(tokens),
                    Token::Whitespace => continue,
                    _ => panic!("Unexpected token")
                };
                block.push(statement);
                tokens = returned_tokens;
            },
            None => break (block, tokens)
        } 
    }
}

#[derive(Debug)]
pub enum Statement {
    Print { value: Expression },
    Let {
        identifier: String,
        value: Expression
    }
}

fn parse_print(mut tokens: IterToken) -> (Statement, IterToken) {
    let (value, returned_tokens) = loop {
        match tokens.next() {
            Some(token) => match token {
                Token::Whitespace => continue,
                token => break parse_expression(token, tokens)
            },
            None => panic!("Unexpected end of program")
        }
    };
    (Statement::Print { value }, returned_tokens)
}

fn parse_let(mut tokens: IterToken) -> (Statement, IterToken) {
    let identifier = loop {
        match tokens.next() {
            Some(token) => match token {
                Token::Whitespace => continue,
                Token::Id(identifier) => break identifier,
                _ => panic!("unexpected token")
            },
            None => panic!("unexpected end of program")
        }
    };
    let (value, returned_tokens) = loop {
        match tokens.next() {
            Some(token) => match token {
                Token::Whitespace => continue,
                token => break parse_expression(token, tokens)
            },
            None => panic!("Unexpected end of program")
        }
    };
    (Statement::Let { identifier, value }, returned_tokens)
}
