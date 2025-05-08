use std::fmt;

use actix_web::{HttpResponse, ResponseError, body::BoxBody};

use crate::application::errors::user_application_error::UserApplicationError;

#[derive(Debug, PartialEq)]
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
            UserHttpError::Constraint(_) => {
                HttpResponse::UnprocessableEntity().json(self.to_string())
            }
            UserHttpError::Internal(_) => {
                HttpResponse::InternalServerError().json(self.to_string())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use actix_web::{ResponseError, body::MessageBody, http::StatusCode};

    use crate::application::errors::user_application_error::UserApplicationError;

    use super::UserHttpError;

    #[test]
    fn display_constraint_error() {
        let err_msg = "Constraint X violated";
        let err = UserHttpError::Constraint(err_msg.to_string());
        let err = err.to_string();

        assert_eq!(
            err,
            format!("A constraint error occurred for the user: {err_msg}")
        );
    }

    #[test]
    fn display_internal_error() {
        let err_msg = "Database error";
        let err = UserHttpError::Internal(err_msg.to_string());
        let err = err.to_string();

        assert_eq!(
            err,
            format!("An internal error occurred for the user: {err_msg}")
        );
    }

    #[test]
    fn from_user_application_conflict_error() {
        let err_msg = "Constraint X violated";
        let application_err = UserApplicationError::Conflict(err_msg.to_string());
        let err: UserHttpError = application_err.into();

        assert_eq!(err, UserHttpError::Constraint(err_msg.to_string()));
    }

    #[test]
    fn from_user_application_internal_error() {
        let err_msg = "Database error";
        let application_err = UserApplicationError::Unexpected(err_msg.to_string());
        let err: UserHttpError = application_err.into();

        assert_eq!(err, UserHttpError::Internal(err_msg.to_string()));
    }

    #[test]
    fn constraint_error_response() -> Result<(), Box<dyn std::error::Error>> {
        let err = UserHttpError::Constraint("Constraint X violated".to_string());

        let result = err.error_response();

        let result_status = result.status();
        let result_body = result.into_body().try_into_bytes().unwrap();
        let result_body = std::str::from_utf8(&result_body)?;

        assert_eq!(result_status, StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(result_body.replace("\"", ""), err.to_string());

        Ok(())
    }

    #[test]
    fn internal_error_response() -> Result<(), Box<dyn std::error::Error>> {
        let err = UserHttpError::Internal("Database error".to_string());

        let result = err.error_response();

        let result_status = result.status();
        let result_body = result.into_body().try_into_bytes().unwrap();
        let result_body = std::str::from_utf8(&result_body)?;

        assert_eq!(result_status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(result_body.replace("\"", ""), err.to_string());

        Ok(())
    }
}
