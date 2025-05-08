use crate::application::errors::user_application_error::UserApplicationError;
use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::UserRepository;
use crate::presentation::dtos::user_dto::CreateUserDTO;

pub struct RegisterUserUseCase<T: UserRepository> {
    user_repo: T,
}

impl<T: UserRepository> RegisterUserUseCase<T> {
    pub fn new(user_repo: T) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, user: CreateUserDTO) -> Result<i32, UserApplicationError> {
        if self.user_repo.exists_by_email(&user.email).await? {
            return Err(UserApplicationError::Conflict(format!(
                "The email {} is already taken",
                user.email
            )));
        }

        let user: User = user.into();

        self.user_repo.save(&user).await.map_err(|err| err.into())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        application::{
            errors::user_application_error::UserApplicationError,
            use_cases::register_user::RegisterUserUseCase,
        },
        domain::{
            entities::user::User, errors::user_repository_error::UserRepositoryError,
            repositories::user_repository::MockUserRepository,
        },
        presentation::dtos::user_dto::CreateUserDTO,
    };

    #[tokio::test]
    async fn execute_user_repository_exists_by_email_error() {
        let mut mock_user_repo = MockUserRepository::new();

        let fake_user = CreateUserDTO {
            name: "Andrew".to_string(),
            email: "andrew@email.com".to_string(),
            phone: "+001122223333".to_string(),
            address: "Dawn St.".to_string(),
        };

        mock_user_repo
            .expect_exists_by_email()
            .times(1)
            .return_const(Err(UserRepositoryError::DatabaseError(
                "Fake Error".to_string(),
            )));

        mock_user_repo.expect_save().times(0);

        let sut = RegisterUserUseCase::new(mock_user_repo);

        let result = sut.execute(fake_user.clone()).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn execute_email_taken_error() {
        let mut mock_user_repo = MockUserRepository::new();

        let email = "andrew@email.com";

        let fake_user = CreateUserDTO {
            name: "Andrew".to_string(),
            email: email.to_string(),
            phone: "+001122223333".to_string(),
            address: "Dawn St.".to_string(),
        };

        mock_user_repo
            .expect_exists_by_email()
            .times(1)
            .return_const(Ok(true));

        mock_user_repo.expect_save().times(0);

        let sut = RegisterUserUseCase::new(mock_user_repo);

        let result = sut.execute(fake_user.clone()).await;

        assert_eq!(
            result,
            Err(UserApplicationError::Conflict(format!(
                "The email {email} is already taken"
            )))
        )
    }

    #[tokio::test]
    async fn execute_user_repository_save_error() {
        let mut mock_user_repo = MockUserRepository::new();

        let fake_user = CreateUserDTO {
            name: "Andrew".to_string(),
            email: "andrew@email.com".to_string(),
            phone: "+001122223333".to_string(),
            address: "Dawn St.".to_string(),
        };

        mock_user_repo
            .expect_exists_by_email()
            .times(1)
            .return_const(Ok(false));

        mock_user_repo.expect_save().times(1).return_const(Err(
            UserRepositoryError::DatabaseError("Fake Error".to_string()),
        ));

        let sut = RegisterUserUseCase::new(mock_user_repo);

        let result = sut.execute(fake_user.clone()).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn execute_ok() -> Result<(), Box<dyn std::error::Error>> {
        let mut mock_user_repo = MockUserRepository::new();

        let email = "andrew@email.com";

        let fake_user = CreateUserDTO {
            name: "Andrew".to_string(),
            email: email.to_string(),
            phone: "+001122223333".to_string(),
            address: "Dawn St.".to_string(),
        };

        let fake_user_entity: User = fake_user.clone().into();

        mock_user_repo
            .expect_exists_by_email()
            .withf(|expected_email: &str| *expected_email == *email)
            .times(1)
            .return_const(Ok(false));

        let new_user_id = 42;

        mock_user_repo
            .expect_save()
            .withf(move |expected_user: &User| *expected_user == fake_user_entity)
            .times(1)
            .return_const(Ok(new_user_id));

        let sut = RegisterUserUseCase::new(mock_user_repo);

        let result = sut.execute(fake_user.clone()).await?;

        assert_eq!(result, new_user_id);

        Ok(())
    }
}
