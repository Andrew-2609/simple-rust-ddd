use crate::{
    application::use_cases::{
        find_user_by_email::FindUserByEmailUseCase, register_user::RegisterUserUseCase,
    },
    infrastructure::repositories::postgres_user_repository::PostgresUserRepository,
    presentation::{
        dtos::user_dto::{CreateUserDTO, LoadedUserDTO},
        errors::user_http_error::UserHttpError,
    },
    schema::users,
};
use actix_web::{
    HttpResponse, ResponseError, get, post,
    web::{self, Path},
};
use diesel::prelude::Insertable;
use serde::Deserialize;

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
    match RegisterUserUseCase::new(repo.into_inner())
        .execute(input.into_inner())
        .await
    {
        Ok(id) => HttpResponse::Ok().json(id),
        Err(err) => UserHttpError::from(err).error_response(),
    }
}

#[get("/{email}")]
pub async fn get_by_email(
    repo: web::Data<PostgresUserRepository>,
    path: Path<String>,
) -> HttpResponse {
    let email = path.into_inner();

    let result = FindUserByEmailUseCase::new(repo.into_inner())
        .execute(email.clone())
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
        Err(err) => UserHttpError::from(err).error_response(),
    }
}
