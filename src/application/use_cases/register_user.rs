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

    pub async fn execute(&self, user: CreateUserDTO) -> Result<i32, String> {
        if self.user_repo.exists_by_email(&user.email).await? {
            return Err(format!("The email {} is already taken", user.email));
        }

        let user: User = user.into();

        self.user_repo.save(&user).await
    }
}
