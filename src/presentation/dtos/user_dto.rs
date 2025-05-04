use serde::{Deserialize, Serialize};

use crate::domain::entities::user::User;

#[derive(Deserialize)]
pub struct CreateUserDTO {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}

impl Into<User> for CreateUserDTO {
    fn into(self) -> User {
        User::new(self.name, self.email, self.phone, self.address)
    }
}

#[derive(Serialize)]
pub struct LoadedUserDTO {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}
