use std::fmt;
use crate::token_type::_TokenType;

pub struct _Token {
    t_type: _TokenType,
    lexeme: Vec<char>,
    line: usize
}

impl _Token {
    pub fn new(t_type: _TokenType, lexeme: Vec<char>, line: usize) -> _Token {
        _Token {
            t_type,
            lexeme,
            line
        }
    }
}

impl fmt::Display for _Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}