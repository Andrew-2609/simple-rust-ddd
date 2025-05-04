use actix_web::{
    HttpResponse, get, post,
    web::{self, Path},
};
use diesel::prelude::Insertable;
use log::error;
use serde::Deserialize;

use crate::{
    application::use_cases::{get_user::GetUserUseCase, register_user::RegisterUserUseCase},
    infrastructure::repositories::postgres_user_repository::PostgresUserRepository,
    presentation::dtos::user_dto::{CreateUserDTO, LoadedUserDTO},
    schema::users,
};

#[derive(Debug, Clone, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}

#[post("")]
pub async fn register_user_handler(
    repo: web::Data<PostgresUserRepository>,
    input: web::Json<CreateUserDTO>,
) -> HttpResponse {
    let input: CreateUserDTO = input.into_inner();

    match RegisterUserUseCase::new(repo.into_inner())
        .execute(input.into())
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            error!("Error registering user! {:?}", err);
            HttpResponse::InternalServerError().body("Please try again...")
        }
    }
}

#[get("/{email}")]
pub async fn get_by_email(
    repo: web::Data<PostgresUserRepository>,
    path: Path<String>,
) -> HttpResponse {
    let email = path.into_inner();

    let result = GetUserUseCase::new(repo.into_inner())
        .get(email.clone())
        .await;

    match result {
        Ok(user) => {
            if let Some(user) = user {
                let loaded_user: Option<LoadedUserDTO> = user.into();
                HttpResponse::Ok().json(loaded_user)
            } else {
                HttpResponse::NotFound().json(format!("User not found by email: {email}"))
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}
