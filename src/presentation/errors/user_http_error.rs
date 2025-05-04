use std::fmt;

use actix_web::{HttpResponse, ResponseError, body::BoxBody};

use crate::application::errors::user_application_error::UserApplicationError;

#[derive(Debug)]
pub enum UserHttpError {
    Constraint(String),
    Internal(String),
}

impl fmt::Display for UserHttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserHttpError::Constraint(msg) => {
                write!(f, "A constraint error occurred for the user: {msg}")
            }
            UserHttpError::Internal(msg) => {
                write!(f, "An internal error occurred for the user: {msg}")
            }
        }
    }
}

impl std::error::Error for UserHttpError {}

impl From<UserApplicationError> for UserHttpError {
    fn from(value: UserApplicationError) -> Self {
        match value {
            UserApplicationError::Conflict(err) => Self::Constraint(err),
            UserApplicationError::Unexpected(err) => Self::Internal(err),
        }
    }
}

impl ResponseError for UserHttpError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            UserHttpError::Constraint(err) => HttpResponse::UnprocessableEntity().json(err),
            UserHttpError::Internal(err) => HttpResponse::InternalServerError().json(err),
        }
    }
}
