use std::fmt;

#[derive(Debug)]
pub enum UserRepositoryError {
    DatabaseError(String),
}

impl fmt::Display for UserRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserRepositoryError::DatabaseError(msg) => {
                write!(f, "A database error occurred when handling users: {msg}")
            }
        }
    }
}

impl std::error::Error for UserRepositoryError {}
