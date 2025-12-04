use std::fmt;
use crate::token_type::TokenType;

pub struct Token {
    t_type: TokenType,
    lexeme: Vec<char>,
    line: usize
}

impl Token {
    pub fn new(t_type: TokenType, lexeme: Vec<char>, line: usize) -> Token {
        Token {
            t_type,
            lexeme,
            line
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}