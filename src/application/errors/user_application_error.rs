use std::fmt;

use crate::domain::errors::user_repository_error::UserRepositoryError;

#[derive(Debug)]
pub enum UserApplicationError {
    Conflict(String),
    Unexpected(String),
}

impl fmt::Display for UserApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserApplicationError::Conflict(msg) => {
                write!(
                    f,
                    "The following conflict occurred when writing a user: {msg}"
                )
            }
            UserApplicationError::Unexpected(msg) => {
                write!(f, "An unexpected error occurred: {msg}")
            }
        }
    }
}

impl std::error::Error for UserApplicationError {}

impl From<UserRepositoryError> for UserApplicationError {
    fn from(value: UserRepositoryError) -> Self {
        match value {
            UserRepositoryError::DatabaseError(err) => Self::Unexpected(err),
        }
    }
}

impl From<UserRepositoryError> for String {
    fn from(value: UserRepositoryError) -> Self {
        value.to_string()
    }
}
