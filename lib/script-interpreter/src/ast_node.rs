use std::any::Any;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::sync::Arc;
use crate::ast::any_expression::AnyExpression;
use crate::ast::expression::Expression;

pub struct AstNode<T: ?Sized + Expression> {
    expression: Box<T>,
    children: Vec<Arc<RefCell<AstNode<dyn Expression>>>>
}

impl <T: Expression> AstNode<T> {
    pub fn new(expression: Box<T>) -> Self {
        AstNode {
            expression,
            children: Vec::with_capacity(50)
        }
    }

    pub fn append_child(&mut self, child: AstNode<AnyExpression>) {
        self.children.push(Arc::new(RefCell::new(child)));
    }

    pub fn expression(&self) -> Box<T> where T: Clone {
        self.expression.clone()
    }
}

impl <T: Expression> Display for AstNode<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "")
    }
}