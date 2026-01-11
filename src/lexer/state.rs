use super::scan::{
    WHITESPACES,
    DIGITS,
    WORD_CHARS,
    SYMBOLS
};

pub enum State {
    DefaultState,
    Word,
    Symbol,
    Number,
    Whitespace,
    RegularString,
    DoubleString
}

impl State {
    pub fn new() -> Self {
        Self::DefaultState
    }
}

impl super::Lexer {
    pub fn change_state(&mut self, character: char) {
        match character {
            '\'' => self.state = State::RegularString,
            '"' => self.state = State::DoubleString,

            _ if WHITESPACES.contains(&character) => {
                self.tokenize_whitespace();
                self.state = State::Whitespace;
            },

            _ => {
                self.add_char(character);
                self.state = match character {
                    _ if DIGITS.contains(&character) => State::Number,
                    _ if WORD_CHARS.contains(&character) => State::Word,
                    _ if SYMBOLS.contains(&character) => State::Symbol,
                    _ => panic!("Unexpected State")
                };
            }
        };
    }
    pub fn end_string(&mut self) {
        self.tokenize_string();
        self.state = State::DefaultState;
    }
}
