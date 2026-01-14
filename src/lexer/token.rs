use std::mem;

impl super::Lexer {
    pub fn tokenize_whitespace(&mut self) {
        self.result.push(Token::Whitespace);
    }
    pub fn tokenize_string(&mut self) {
        self.result.push(Token::StringToken(mem::take(&mut self.current_token)));
        self.current_token = String::new();
    }
    pub fn tokenize_word(&mut self) {
        self.result.push(match self.current_token.as_str() {
            "sit" => Token::Let,
            "echo" => Token::Print,
            _ => Token::Id(mem::take(&mut self.current_token))
        });
        self.current_token = String::new();
    }
    pub fn tokenize_symbol(&mut self) {
        self.result.push(match self.current_token.as_str() {
            "==" => Token::EqualOperator,
            "!=" => Token::NotEqualOperator,
            "+" => Token::AddOperator,
            "-" => Token::SubtractOperator,
            "*" => Token::MultiplyOperator,
            "/" => Token::DivideOperator,
            _ => panic!("Unexpected symbol sequence")
        });
        self.current_token = String::new();
    }
    pub fn tokenize_number(&mut self) {
        self.result.push(Token::Number(
                mem::take(&mut self.current_token)
                .parse::<u32>()
                .unwrap_or_else(|_error| panic!("Tried to turn into number a sequence that wasn't entirely a number: {}", self.current_token))
        ));
        self.current_token = String::new();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Let,
    Print,
    EqualOperator,
    NotEqualOperator,    
    AddOperator,
    SubtractOperator,
    MultiplyOperator,
    DivideOperator,
    Id(String),
    StringToken(String),
    Number(u32),
    Whitespace,
    EndOfProgram
}
