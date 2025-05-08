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

#[cfg(test)]
mod test {
    use mockall::predicate::eq;

    use crate::{
        application::use_cases::find_user_by_email::FindUserByEmailUseCase,
        domain::{
            entities::user::User, errors::user_repository_error::UserRepositoryError,
            repositories::user_repository::MockUserRepository,
        },
    };

    #[tokio::test]
    async fn execute_user_repository_error() {
        let mut mock_user_repository = MockUserRepository::new();

        mock_user_repository
            .expect_find_by_email()
            .times(1)
            .return_const(Err(UserRepositoryError::DatabaseError(
                "Fake Error".to_string(),
            )));

        let sut = FindUserByEmailUseCase::new(mock_user_repository);

        let result = sut.execute("any@email.com".to_string()).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn execute_ok() -> Result<(), Box<dyn std::error::Error>> {
        let mut mock_user_repository = MockUserRepository::new();

        let fake_user = User::restore(
            42,
            "Andrew".to_string(),
            "andrew@email.com".to_string(),
            "+001133334444".to_string(),
            "Dawn St.".to_string(),
        )?;

        mock_user_repository
            .expect_find_by_email()
            .with(eq(fake_user.email.clone()))
            .times(1)
            .return_const(Ok(Some(fake_user.clone())));

        let sut = FindUserByEmailUseCase::new(mock_user_repository);

        let result = sut.execute(fake_user.email.clone()).await?;

        assert_eq!(result, Some(fake_user));

        Ok(())
    }
}
