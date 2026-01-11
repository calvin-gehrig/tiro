pub const DIGITS : [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
pub const WORD_CHARS : [char; 63] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '_'
];
pub const WHITESPACES : [char; 4] = [' ', '\t', '\n', '\r'];
pub const SYMBOLS : [char; 6] = ['=', '!', '+', '-', '/', '*'];

pub fn scan_word(character: char, lexer: &mut super::Lexer) {
    if WORD_CHARS.contains(&character) {
        lexer.add_char(character);
    } else {
        lexer.tokenize_word();
        lexer.change_state(character);
    }
}

pub fn scan_symbol(character: char, lexer: &mut super::Lexer) {
    if SYMBOLS.contains(&character) {
        lexer.add_char(character);
    } else {
        lexer.tokenize_symbol();
        lexer.change_state(character);
    }
}

pub fn scan_whitespace(character: char, lexer: &mut super::Lexer) {
    if !WHITESPACES.contains(&character) {
        lexer.change_state(character);
    }
}

pub fn scan_string(character: char, delimiter: char, lexer: &mut super::Lexer) {
    if delimiter == character {
        lexer.end_string();
    } else {
        lexer.add_char(character);
    }
}

pub fn scan_number(character: char, lexer: &mut super::Lexer) {
    if DIGITS.contains(&character) {
        lexer.add_char(character);
    } else {
        lexer.tokenize_number();
        lexer.change_state(character);
    }
}
