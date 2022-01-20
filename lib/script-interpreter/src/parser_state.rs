use crate::token::{Token, TokenType};

pub struct ParserState {
    tokens: &'static Vec<Token>,
    current: usize
}

impl ParserState {
    pub fn token_match(&mut self, token_types: Vec::<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    pub fn consume(&mut self, token_type: TokenType, message: String) -> Result<&Token, String> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        Err(message)
    }

    pub fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    pub fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    pub fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1
        }
        self.previous()
    }

    pub fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    pub fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }
}