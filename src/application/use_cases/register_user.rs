use crate::domain::entities::user::User;
use crate::domain::{
    repositories::user_repository::UserRepository, services::user_service::UserService,
};

pub struct RegisterUserUseCase<T: UserRepository> {
    user_service: UserService<T>,
}

impl<T: UserRepository> RegisterUserUseCase<T> {
    pub fn new(user_repository: T) -> Self {
        let user_service = UserService::new(user_repository);
        Self { user_service }
    }

    pub async fn execute(&self, user: User) -> Result<i32, String> {
        self.user_service.register_user(user).await
    }
}
