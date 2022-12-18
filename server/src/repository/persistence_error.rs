use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use sqlx::error::DatabaseError;

#[derive(Debug)]
pub struct PersistenceError {
    pub message: String
}
impl PersistenceError {
    pub fn new(message: String) -> Self {
        Self {
            message
        }
    }
}
impl StdError for PersistenceError {}

impl Display for PersistenceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl DatabaseError for PersistenceError {
    fn message(&self) -> &str {
        self.message.as_str()
    }

    fn as_error(&self) -> &(dyn std::error::Error + Send + Sync + 'static) {
        self
    }

    fn as_error_mut(&mut self) -> &mut (dyn std::error::Error + Send + Sync + 'static) {
        self
    }

    fn into_error(self: Box<Self>) -> Box<dyn std::error::Error + Send + Sync + 'static> {
        self
    }
}