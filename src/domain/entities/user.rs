use diesel::{Queryable, prelude::Insertable};

use crate::{
    domain::value_objects::id::ID, presentation::dtos::user_dto::CreateUserDTO, schema::users,
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

impl From<CreateUserDTO> for User {
    fn from(value: CreateUserDTO) -> Self {
        Self::new(value.name, value.email, value.phone, value.address)
    }
}
