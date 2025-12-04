use crate::token::Token;
use crate::token_type::TokenType::*;

pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize
}

impl Scanner {
    pub fn new(content: String) -> Scanner {
        Scanner {
            source : content,
            start: 0,
            current: 0,
            line: 0
        }
    }

    pub fn scan_tokens(&mut self) {
        
        while self.current < self.source.len() {
            self.start = self.current;

        }

        let lexeme = vec!{'\0'};
        let token = Token::new(EOF, lexeme, self.line);
    }
}