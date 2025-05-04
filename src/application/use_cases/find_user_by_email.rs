use crate::{
    application::errors::user_application_error::UserApplicationError,
    domain::{entities::user::User, repositories::user_repository::UserRepository},
};

pub struct FindUserByEmailUseCase<T: UserRepository> {
    user_repo: T,
}

impl<T: UserRepository> FindUserByEmailUseCase<T> {
    pub fn new(user_repo: T) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, email: String) -> Result<Option<User>, UserApplicationError> {
        self.user_repo
            .find_by_email(email)
            .await
            .map_err(|err| err.into())
    }
}
