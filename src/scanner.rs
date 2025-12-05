use crate::token::_Token;
use crate::token_type::_TokenType::*;

pub struct _Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize
}

impl _Scanner {
    pub fn new(content: &String) -> _Scanner {
        _Scanner {
            source : content.to_string(),
            start: 0,
            current: 0,
            line: 0
        }
    }

    pub fn scan_tokens(&mut self) {
        
        while self.current < self.source.len() {
            self.start = self.current;
            self.current += 1;
        }

        let lexeme = vec!{'\0'};
        let token = _Token::new(EOF, lexeme, self.line);
    }
}