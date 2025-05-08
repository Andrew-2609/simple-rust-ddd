use std::fmt;

#[derive(Debug, PartialEq)]
pub enum UserEntityError {
    InvalidId(i32),
}

impl fmt::Display for UserEntityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserEntityError::InvalidId(user_id) => {
                write!(f, "An invalid ID was given for a user: {user_id}")
            }
        }
    }
}

impl std::error::Error for UserEntityError {}

#[cfg(test)]
mod test {
    use super::UserEntityError;

    #[test]
    fn display() {
        let user_id: i32 = 0;
        let err = UserEntityError::InvalidId(user_id);
        let err = err.to_string();

        assert_eq!(
            err,
            format!("An invalid ID was given for a user: {user_id}")
        );
    }
}
