use crate::domain::{entities::user::User, repositories::user_repository::UserRepository};

#[derive(Clone)]
pub struct UserService<T: UserRepository> {
    user_repo: T,
}

impl<T: UserRepository> UserService<T> {
    pub fn new(user_repo: T) -> Self {
        UserService { user_repo }
    }

    pub async fn register_user(&self, user: User) -> Result<i32, String> {
        self.user_repo.save(&user).await
    }

    pub async fn get_by_email(&self, email: String) -> Result<Option<User>, String> {
        self.user_repo.find_by_email(email).await
    }
}
