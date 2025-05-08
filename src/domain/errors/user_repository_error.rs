use std::fmt;

#[derive(Debug, Clone)]
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

#[cfg(test)]
mod test {
    use super::UserRepositoryError;

    #[test]
    fn display() {
        let error_msg = "Connection lost";
        let err = UserRepositoryError::DatabaseError(error_msg.to_string());
        let err = err.to_string();

        assert_eq!(
            err,
            "A database error occurred when handling users: ".to_owned() + error_msg
        );
    }
}
