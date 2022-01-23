use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

use crate::ast::expression::Expression;
use crate::ast::root_expression::RootExpression;

pub struct Tree {
    nodes: Vec<AstNode<dyn Expression>>,
}

pub struct AstNode<T: ?Sized + Expression> {
    id: usize,
    expression: Box<T>,
    parent_id: Option<usize>,
    first_child_id: Option<usize>,
    last_child_id: Option<usize>,
}

impl Tree {
    pub fn new(init_capacity: usize) -> Self {
        Tree {
            nodes: Vec::with_capacity(init_capacity)
        }
    }

    pub fn new_node<T: 'static + Expression>(&mut self, expression: T, parent_id: usize) -> usize {
        let id = self.nodes.len();
        self.nodes.push(
            AstNode {
                id,
                expression: Box::new(expression),
                parent_id: Some(parent_id),
                first_child_id: None,
                last_child_id: None
            }
        );
        id
    }

    pub fn new_root_node(&mut self, expression: RootExpression) -> usize {
        self.nodes.push(
            AstNode {
                id: 0,
                expression: Box::new(expression),
                parent_id: None,
                first_child_id: None,
                last_child_id: None
            }
        );
        0
    }
}

impl<T: Expression> AstNode<T> {

    pub fn expression(&self) -> Box<T> where T: Clone {
        self.expression.clone()
    }
}

impl<T: Expression> Display for AstNode<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "")
    }
}