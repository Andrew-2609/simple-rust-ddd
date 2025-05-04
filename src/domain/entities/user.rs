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

#[cfg(test)]
mod test {
    use crate::{
        domain::{entities::user::User, value_objects::id::ID},
        presentation::dtos::user_dto::CreateUserDTO,
    };

    #[test]
    fn new() {
        let name = "Andrew";
        let email = "andrew@email.com";
        let phone = "+550011111-2222";
        let address = "Dawn St.";

        let user = User::new(
            name.to_string(),
            email.to_string(),
            phone.to_string(),
            address.to_string(),
        );

        assert_eq!(user.id, ID::New);
        assert_eq!(user.name, name);
        assert_eq!(user.email, email);
        assert_eq!(user.phone, phone);
        assert_eq!(user.address, address);
    }

    #[test]
    fn from_create_user_dto() {
        let dto = CreateUserDTO {
            name: String::from("Andrew"),
            email: String::from("andrew@email.com"),
            phone: String::from("+550011111-2222"),
            address: String::from("Dawn St."),
        };

        let user: User = dto.clone().into();

        assert_eq!(user.id, ID::New);
        assert_eq!(user.name, dto.name);
        assert_eq!(user.email, dto.email);
        assert_eq!(user.phone, dto.phone);
        assert_eq!(user.address, dto.address);
    }
}
