use diesel::{Queryable, prelude::Insertable};
use log::warn;

use crate::{
    domain::value_objects::id::ID, presentation::dtos::user_dto::LoadedUserDTO, schema::users,
};

#[derive(Debug, Clone, Insertable, Queryable)]
#[diesel(table_name = users)]
pub struct User {
    #[diesel(serialize_as = Option<i32>, deserialize_as = i32)]
    pub id: ID,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}

impl User {
    pub fn new(name: String, email: String, phone: String, address: String) -> Self {
        Self {
            id: ID::New,
            name,
            email,
            phone,
            address,
        }
    }
}

impl Into<Option<LoadedUserDTO>> for User {
    fn into(self) -> Option<LoadedUserDTO> {
        match self.id {
            ID::Existing(id) => Some(LoadedUserDTO {
                id,
                name: self.name,
                email: self.email,
                phone: self.phone,
                address: self.address,
            }),
            ID::New => {
                warn!("trying to initialize LoadedUserDTO without ID");
                None
            }
        }
    }
}
