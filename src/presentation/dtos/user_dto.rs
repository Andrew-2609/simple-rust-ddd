use serde::{Deserialize, Serialize};

use crate::domain::{entities::user::User, value_objects::id::ID};

#[derive(Deserialize, Clone)]
pub struct CreateUserDTO {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}

#[derive(Serialize, PartialEq)]
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

#[cfg(test)]
mod test {
    use crate::domain::entities::user::User;
    use crate::domain::value_objects::id::ID;
    use crate::presentation::dtos::user_dto::LoadedUserDTO;

    #[test]
    fn from_user_into_optional_loaded_user_dto() {
        let id = 42;
        let name = "Andrew";
        let email = "andrew@email.com";
        let phone = "+550011111-2222";
        let address = "Dawn St.";

        let new_user = User::new(
            name.to_string(),
            email.to_string(),
            phone.to_string(),
            address.to_string(),
        );

        let loaded_user_dto: Option<LoadedUserDTO> = new_user.clone().into();

        assert!(loaded_user_dto.is_none());

        let mut existing_user = new_user;
        existing_user.id = ID::Existing(id);

        let loaded_user_dto: Option<LoadedUserDTO> = existing_user.into();
        let loaded_user_dto = loaded_user_dto.unwrap();

        assert_eq!(loaded_user_dto.id, id);
        assert_eq!(loaded_user_dto.name, name);
        assert_eq!(loaded_user_dto.email, email);
        assert_eq!(loaded_user_dto.phone, phone);
        assert_eq!(loaded_user_dto.address, address);
    }
}
