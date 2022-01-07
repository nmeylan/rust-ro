use std::fmt::{Display, Formatter};
use crate::token::{Token, TokenType};

pub struct Scanner {
    source: Vec<u8>,
    lexeme_start_offset: usize,
    current_char_offset: usize,
    current_line: u32,
    current_line_start_offset: usize,
    current_pos_in_line: usize,
    tokens: Vec<Token>,
}

#[derive(Debug)]
pub struct ScanError {
    line: u32,
    message: String,
    line_content: String,
    lexeme_content: String,
}

impl Display for ScanError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_str())
    }
}

impl Scanner {
    pub fn new(source: Vec<u8>) -> Scanner {
        Scanner {
            source,
            lexeme_start_offset: 0,
            current_char_offset: 0,
            current_line: 0,
            current_line_start_offset: 0,
            current_pos_in_line: 0,
            tokens: Vec::with_capacity(2048)
        }
    }

    pub fn scan(&mut self) -> Result<Vec<Token>, ScanError> {
        while !self.is_at_end() {
            self.lexeme_start_offset = self.current_char_offset;
            let maybe_scan_error = self.scan_token();
            if maybe_scan_error.is_some() {
                return Result::Err(maybe_scan_error.unwrap());
            }
        }
        return Result::Ok(self.tokens.clone());
    }

    fn scan_token(&mut self) -> Option<ScanError> {
        let char = self.advance_char();
        match char {
            b'(' => self.add_token(TokenType::LeftParen),
            b')' => self.add_token(TokenType::RightParen),
            b'{' => self.add_token(TokenType::LeftBrace),
            b'}' => self.add_token(TokenType::RightBrace),
            b'[' => self.add_token(TokenType::LeftBracket),
            b']' => self.add_token(TokenType::RightBracket),
            b',' => self.add_token(TokenType::Comma),
            b':' => self.add_token(TokenType::Colon),
            b';' => self.add_token(TokenType::SemiColon),
            b'%' => self.add_token(TokenType::Percent),
            b'^' => self.add_token(TokenType::Caret),
            b'~' => self.add_token(TokenType::Tilde),
            b'?' => self.add_token(TokenType::QuestionMark),
            b'@' => self.add_token(TokenType::At),
            b'\'' => self.add_token(TokenType::Quote),
            b'$' => {
                if self.does_next_match(b'@') {
                    self.add_token(TokenType::DollarAt)
                } else {
                    self.add_token(TokenType::Dollar)
                }
            },
            b'.' => {
                if self.does_next_match(b'@') {
                    self.add_token(TokenType::DotAt)
                } else {
                    self.add_token(TokenType::Dot)
                }
            },
            b'-' => {
                if self.does_next_match(b'-') {
                    self.add_token(TokenType::DecrementOp)
                } else {
                    self.add_token(TokenType::Minus)
                }
            },
            b'+' => {
                if self.does_next_match(b'+') {
                    self.add_token(TokenType::IncrementOp)
                } else {
                    self.add_token(TokenType::Plus)
                }
            },
            b'/' => {
                if self.does_next_match(b'/') {
                    // Inside a single line comment, consume all char of the comment
                    while self.peek_next_char() != b'\n' && !self.is_at_end() {
                        self.advance_char();
                    }
                    self.advance_char();
                } else if self.does_next_match(b'*') {
                    return self.consume_multiline_comment();
                } else {
                    self.add_token(TokenType::Slash)
                }
            },
            b'*' => {
                if self.does_next_match(b'/') {
                    self.add_token(TokenType::StarSlash)
                } else {
                    self.add_token(TokenType::Star)
                }
            },
            b'|' => {
                if self.does_next_match(b'|') {
                    self.add_token(TokenType::OrOp)
                } else {
                    self.add_token(TokenType::LogicalOr)
                }
            },
            b'&' => {
                if self.does_next_match(b'&') {
                    self.add_token(TokenType::AndOp)
                } else {
                    self.add_token(TokenType::LogicalAnd)
                }
            },
            b'#' => {
                if self.does_next_match(b'#') {
                    self.add_token(TokenType::DoubleSharp)
                } else {
                    self.add_token(TokenType::Sharp)
                }
            },
            b'!' => {
                if self.does_next_match(b'=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            },
            b'=' => {
                if self.does_next_match(b'=') {
                    self.add_token(TokenType::DoubleEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            },
            b'<' => {
                if self.does_next_match(b'=') {
                    self.add_token(TokenType::LeftCaretEqual)
                } else if self.does_next_match(b'<') {
                    self.add_token(TokenType::DoubleLeftCaret)
                } else {
                    self.add_token(TokenType::LeftCaret)
                }
            },
            b'>' => {
                if self.does_next_match(b'=') {
                    self.add_token(TokenType::RightCaretEqual)
                } else if self.does_next_match(b'>') {
                    self.add_token(TokenType::DoubleRightCaret)
                } else {
                    self.add_token(TokenType::RightCaret)
                }
            },
            b' ' | b'\t' | b'\r' => (), // skipping blank space, carriage return, etc..
            b'\n' => self.add_new_line(),
            b'"' => return self.string(),
            _ => {
                if char.is_ascii_digit() {
                    while self.peek().is_ascii_digit() {
                        self.advance_char();
                    }
                    self.add_token(TokenType::Number(self.string_from(self.lexeme_start_offset, self.current_char_offset).parse::<i32>().unwrap()));
                } else if char.is_ascii_alphabetic() {
                    self.identifier();
                } else {
                    return Some(self.build_error(format!("scanning {}. This char is not recognized", char).as_str()));
                }
            }
        }
        None
    }

    fn add_new_line(&mut self) {
        self.current_line += 1;
        self.current_line_start_offset = self.current_char_offset;
        self.current_pos_in_line = 0;
    }

    fn consume_multiline_comment(&mut self) -> Option<ScanError> {
        loop {
            if self.is_at_end() {
                return Some(self.build_error("scanning multiline comment. Reached end of buffer without encountering */ (start + slash) to end multiline comment."));
            }
            let mut previous_char = self.peek();
            if previous_char == b'\n' {
                self.add_new_line();
            }
            if previous_char == b'*' && self.peek_next_char() == b'/' {
                self.advance_char();
                break;
            }
            previous_char = self.advance_char();
        }
        self.advance_char();
        None
    }

    fn string(&mut self) -> Option<ScanError> {
        let mut previous_char: u8 = b'"';
        while !(self.peek() == b'"' && previous_char != b'\\') // double-quote in a string is valid but must be escaped
            && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.add_new_line();
            }
            previous_char = self.advance_char();
        }
        if self.is_at_end() {
            return Some(self.build_error("scanning string. Reached end of buffer without encountering \" (double-quote) to close string"));
        }
        // advance to consume closing "
        self.advance_char();
        self.add_token(TokenType::String(self.string_from(self.lexeme_start_offset + 1, self.current_char_offset - 1))); // +1 -1 are there to remove surrounding double quote
        None
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric( ){
            self.advance_char();
        }
        if self.does_next_match(b':') {
            self.add_token(TokenType::Label(self.string_from(self.lexeme_start_offset, self.current_char_offset - 1))); // -1 is there to remove leading colon
        } else {
            let identifier_text = self.str_from(self.lexeme_start_offset, self.current_char_offset);
            let maybe_keyword = TokenType::from_keyword(identifier_text);
            if maybe_keyword.is_some() {
                self.add_token(maybe_keyword.unwrap());
            } else {
                self.add_token(TokenType::Identifier(identifier_text.to_string()));
            }
        }

    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token {
            token_type,
            lexeme: self.string_from(self.lexeme_start_offset, self.current_char_offset),
            line: self.current_line
        })
    }

    fn advance_char(&mut self) -> u8 {
        let char = self.source[self.current_char_offset];
        if self.current_char_offset < self.source.len() {
            self.current_char_offset += 1;
            self.current_pos_in_line += 1;
        }
        char
    }

    fn string_from(&self, start: usize, end: usize) -> String {
        self.str_from(start, end).to_string()
    }

    fn str_from(&self, start: usize, end: usize) -> &str {
        std::str::from_utf8(&self.source[start..end]).unwrap()
    }

    fn does_next_match(&mut self, expectation: u8) -> bool {
        if self.is_at_end() || self.peek() != expectation {
            return false;
        }
        self.current_char_offset += 1;
        true
    }

    fn build_error(&self, message: &str) -> ScanError {
        ScanError {
            line: self.current_line as u32,
            message:  format!("Error while {}", message),
            line_content: self.string_from(self.current_line_start_offset, self.current_line_start_offset + self.current_pos_in_line),
            lexeme_content: self.string_from(self.lexeme_start_offset, self.current_char_offset)
        }
    }

    #[inline]
    fn is_at_end(&self) -> bool {
        self.current_char_offset >= self.source.len()
    }

    #[inline]
    fn peek(&self) -> u8 {
        if self.is_at_end() {
            return b'\0';
        }
        self.source[self.current_char_offset]
    }

    fn peek_next_char(&mut self) -> u8 {
        if self.current_char_offset < self.source.len() - 1 {
            return self.source[self.current_char_offset + 1];
        }
        b'\0'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multilines_comments_should_not_generate_token() {
        // Given
        let code = r#"
        /*
        * multiline
        */
        "#;
        let mut scanner = Scanner::new(code.as_bytes().to_vec());
        // When
        let tokens = scanner.scan().unwrap_or_else(|e| panic!("{}", e));
        // Then
        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn single_line_comment_should_not_generate_token() {
        // Given
        let code = r#"
        // I am a single line comment
        "#;
        let mut scanner = Scanner::new(code.as_bytes().to_vec());
        // When
        let tokens = scanner.scan().unwrap_or_default();
        // Then
        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn string() {
        // Given
        let code = r#"
        .@str = "i'm a string";
        "#;
        let mut scanner = Scanner::new(code.as_bytes().to_vec());
        // When
        let tokens = scanner.scan().unwrap_or_default();
        // Then
        assert_eq!(tokens.len(), 5);
        let maybe_token_string = tokens.iter().find(|token| token.lexeme == "\"i'm a string\"".to_string());
        assert!(maybe_token_string.is_some());
        let token_string = maybe_token_string.unwrap();
        assert_eq!(token_string.token_type, TokenType::String("i'm a string".to_string()));
    }

    #[test]
    fn string_with_escaped_double_quote() {
        // Given
        let code = r#"
        .@str = "i'm a \"string\"";
        "#;
        let mut scanner = Scanner::new(code.as_bytes().to_vec());
        // When
        let tokens = scanner.scan().unwrap_or_default();
        // Then
        assert_eq!(tokens.len(), 5);
        let maybe_token_string = tokens.iter().find(|token| token.lexeme ==  "\"i'm a \\\"string\\\"\"".to_string());
        assert!(maybe_token_string.is_some());
        let token_string = maybe_token_string.unwrap();
        assert_eq!(token_string.token_type, TokenType::String("i'm a \\\"string\\\"".to_string()));
    }

    #[test]
    fn string_unfinished_should_generate_an_error() {
        // Given
        let code = r#"
        .@str = "i'm a string;
        "#;
        let mut scanner = Scanner::new(code.as_bytes().to_vec());
        // When
        let error = scanner.scan().unwrap_err();
        // Then
        println!("{:?}", error);
    }

    #[test]
    fn keywords() {
        // Given
        let code = r#"
        .@x = 2;
        if (.@x <= 100) {
          .@x = 99;
        }
        return .@x+1;
        "#;
        let mut scanner = Scanner::new(code.as_bytes().to_vec());
        // When
        let tokens = scanner.scan().unwrap_or_default();
        // Then
        assert_eq!(tokens.len(), 25);
        assert!(tokens.iter().find(|token| token.token_type ==  TokenType::If).is_some());
        assert!(tokens.iter().find(|token| token.token_type ==  TokenType::Return).is_some());
    }

    #[test]
    fn label() {
        // Given
        let code = r#"
        goto Step1
        Step1:
        return .@x+1;
        "#;
        let mut scanner = Scanner::new(code.as_bytes().to_vec());
        // When
        let tokens = scanner.scan().unwrap_or_default();
        // Then
        assert_eq!(tokens.len(), 9);
        assert!(tokens.iter().find(|token| token.token_type ==  TokenType::Label("Step1".to_string())).is_some());
        assert!(tokens.iter().find(|token| token.token_type ==  TokenType::Goto).is_some());
    }
}