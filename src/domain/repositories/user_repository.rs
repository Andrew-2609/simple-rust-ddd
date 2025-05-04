use crate::domain::entities::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn save(&self, user: &User) -> Result<i32, String>;
    async fn exists_by_email(&self, email: &str) -> Result<bool, String>;
    async fn find_by_email(&self, email: String) -> Result<Option<User>, String>;
}
