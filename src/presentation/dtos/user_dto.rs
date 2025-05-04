use serde::{Deserialize, Serialize};

use crate::domain::{entities::user::User, value_objects::id::ID};

#[derive(Deserialize)]
pub struct CreateUserDTO {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}

#[derive(Serialize)]
pub struct LoadedUserDTO {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}

impl From<User> for Option<LoadedUserDTO> {
    fn from(value: User) -> Self {
        match value.id {
            ID::Existing(id) => Self::Some(LoadedUserDTO {
                id,
                name: value.name,
                email: value.email,
                phone: value.phone,
                address: value.address,
            }),
            ID::New => None,
        }
    }
}
