use std::cell::{Ref, RefCell};
use std::ops::Deref;
use std::sync::Arc;
use crate::ast::expression::Expression;
use crate::ast_node::AstNode;
use crate::token::{Token, TokenType};

pub struct ParserState {
    tokens: &'static Vec<Token>,
    root_node: Option<AstNode>,
    current_node: Option<Arc<RefCell<AstNode>>>,
    current: usize,
}

impl ParserState {
    pub fn new(tokens: &'static Vec<Token>) -> Self {
        ParserState {
            tokens,
            root_node: None,
            current_node: None,
            current: 0
        }
    }

    pub fn set_root_node(&mut self, expression: Box<dyn Expression>) {
        self.root_node = Some(AstNode::new_expression(expression));
    }

    pub fn consume(&mut self, token_type: TokenType) -> bool {
        let current_node_arc = self.current_node.as_ref().unwrap().clone();
        if self.check(token_type.clone()) {
            let mut current_node_mut_ref = current_node_arc.borrow_mut();
            current_node_mut_ref.append_child(Box::new(RefCell::new(AstNode::new_token( self.advance().clone()))));
            return true
        }
        println!("Expected {:?} after {}", token_type, current_node_arc.borrow());
        return false;
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