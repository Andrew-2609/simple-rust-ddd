use crate::domain::{entities::user::User, errors::user_repository_error::UserRepositoryError};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn save(&self, user: &User) -> Result<i32, UserRepositoryError>;
    async fn exists_by_email(&self, email: &str) -> Result<bool, UserRepositoryError>;
    async fn find_by_email(&self, email: String) -> Result<Option<User>, UserRepositoryError>;
}
