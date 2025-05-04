use crate::domain::entities::user::User;
use crate::presentation::handlers::user_handler::NewUser;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn save(&self, user: &NewUser) -> Result<(), diesel::result::Error>;
    async fn find_by_email(&self, email: String) -> Option<User>;
}
