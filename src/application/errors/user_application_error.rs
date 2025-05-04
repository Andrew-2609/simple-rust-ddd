use std::fmt;

use crate::domain::errors::user_repository_error::UserRepositoryError;

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod test {
    use crate::{
        application::errors::user_application_error::UserApplicationError,
        domain::errors::user_repository_error::UserRepositoryError,
    };

    #[test]
    fn user_application_error_conflict_display() {
        let err_msg = "email already taken";
        let err = UserApplicationError::Conflict(err_msg.to_string());
        let err = err.to_string();

        assert_eq!(
            err,
            "The following conflict occurred when writing a user: ".to_owned() + err_msg
        );
    }

    #[test]
    fn user_application_error_unexpected_display() {
        let err_msg = "database error";
        let err = UserApplicationError::Unexpected(err_msg.to_string());
        let err = err.to_string();

        assert_eq!(err, "An unexpected error occurred: ".to_owned() + err_msg);
    }

    #[test]
    fn user_application_error_from_user_repository_error() {
        let err_msg = "database error";
        let repo_err = UserRepositoryError::DatabaseError(err_msg.to_string());
        let err: UserApplicationError = repo_err.into();

        assert_eq!(err, UserApplicationError::Unexpected(err_msg.to_string()));
    }
}
