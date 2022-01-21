use std::cell::{RefCell};
use std::sync::Arc;
use crate::ast::expression::Expression;
use crate::ast::root_expression::RootExpression;
use crate::ast_node::AstNode;
use crate::token::{Token, TokenType};

pub struct ParserState {
    tokens: &'static Vec<Token>,
    root_node: Option<AstNode<RootExpression>>,
    current: usize,
}

impl ParserState {
    pub fn new(tokens: &'static Vec<Token>) -> Self {
        ParserState {
            tokens,
            root_node: None,
            current: 0
        }
    }

    pub fn set_root_node(&mut self, expression: Box<RootExpression>) {
        self.root_node = Some(AstNode::new(expression));
    }

    pub fn consume(&mut self, token_type: TokenType) -> Result<Token, String> {
        if self.check(token_type.clone()) {
            return Ok(self.advance().clone());
        }
        Err(format!("Expected {:?}", token_type))
    }

    pub fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(&self.peek().token_type) ==  std::mem::discriminant(&token_type)
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