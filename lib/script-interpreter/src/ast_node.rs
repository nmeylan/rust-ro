use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use crate::ast::expression::Expression;
use crate::token::Token;

pub struct AstNode {
    expression: Option<Box<dyn Expression>>,
    token: Option<Token>,
    is_terminal: bool,
    children: Vec<Box<RefCell<AstNode>>>
}

impl AstNode {
    pub fn new_expression(expression: Box<dyn Expression>) -> Self {
        AstNode {
            expression: Some(expression),
            token: None,
            is_terminal: false,
            children: Vec::with_capacity(50)
        }
    }
    pub fn new_token(token: Token) -> Self {
        AstNode {
            expression: None,
            token: Some(token),
            is_terminal: true,
            children: vec![]
        }
    }

    pub fn append_child(&mut self, child: Box<RefCell<AstNode>>) {
        self.children.push(child);
    }
}

impl Display for AstNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_terminal {
            return write!(f, "{}", self.token.as_ref().unwrap());
        }
        return write!(f, "")
    }
}