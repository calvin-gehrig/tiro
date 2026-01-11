use std::mem;

mod state;
use state::State;

mod scan;
use scan::{
    scan_word,
    scan_symbol,
    scan_number,
    scan_whitespace,
    scan_string
};

mod token;
pub use token::Token;

pub fn tokenize(raw_program: String) -> Vec<Token> {
    let mut lexer = Lexer::new();

    for character in raw_program.chars() {
        match lexer.state {
            State::DefaultState => lexer.change_state(character),
            State::Word => scan_word(character, &mut lexer),
            State::Symbol => scan_symbol(character, &mut lexer),
            State::Number => scan_number(character, &mut lexer),
            State::Whitespace => scan_whitespace(character, &mut lexer),
            State::RegularString => scan_string(character, '\'', &mut lexer),
            State::DoubleString => scan_string(character, '"', &mut lexer)
        }

    }
    mem::take(&mut lexer.result)
}

struct Lexer {
    state : State,
    current_token : String,
    result : Vec<Token>
}

impl Lexer {
    fn new() -> Self {
        Self {
            state : State::new(),
            current_token: String::new(),
            result: Vec::new()
        }
    }
    fn add_char(&mut self, character: char) {
        self.current_token.push(character);
    }
}
