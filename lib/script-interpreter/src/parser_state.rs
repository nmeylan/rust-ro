use std::sync::Arc;

use crate::ast::expression::Expression;
use crate::ast::root_expression::RootExpression;
use crate::ast_node::{AstNode, Tree};
use crate::token::{Token, TokenType};

pub struct ParserState {
    tokens: Arc<Vec<Token>>,
    root_node: Option<AstNode<RootExpression>>,
    pub current_token: usize,
    tree: Tree
}

impl ParserState {
    pub fn new(tokens: Arc<Vec<Token>>) -> Self {
        ParserState {
            tokens,
            root_node: None,
            current_token: 0,
            tree: Tree::new(500)
        }
    }

    pub fn add_node<T: 'static + Expression>(&mut self, expression: T, parent_id: usize) -> usize {
        self.tree.new_node::<T>(expression, parent_id)
    }

    pub fn add_root_node(&mut self, expression: RootExpression) -> usize {
        self.tree.new_root_node(expression)
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
        std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(&token_type)
    }

    pub fn set_current_token(&mut self, offset: usize) {
        self.current_token = offset;
    }

    pub fn is_at_end(&self) -> bool {
        self.tokens.get(self.current_token).is_none()
        || self.peek().token_type == TokenType::Eof
    }

    pub fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current_token += 1;
        }
        self.previous()
    }

    pub fn peek(&self) -> &Token {
        self.tokens.get(self.current_token).unwrap()
    }

    pub fn previous(&self) -> &Token {
        self.tokens.get(self.current_token - 1).unwrap()
    }
}