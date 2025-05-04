use crate::domain::errors::user_repository_error::UserRepositoryError;
use crate::schema::users::dsl::{email, id, users};
use crate::{
    domain::{entities::user::User, repositories::user_repository::UserRepository},
    infrastructure::db::connection::{DBPool, establish_connection},
    schema,
};
use async_trait::async_trait;
use diesel::dsl::exists;
use diesel::{prelude::*, select};
use std::sync::Arc;

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: DBPool,
}

impl PostgresUserRepository {
    pub fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is missing");
        Self {
            pool: establish_connection(&database_url),
        }
    }
}

impl From<diesel::result::Error> for UserRepositoryError {
    fn from(value: diesel::result::Error) -> Self {
        UserRepositoryError::DatabaseError(value.to_string())
    }
}

#[async_trait]
impl UserRepository for Arc<PostgresUserRepository> {
    async fn save(&self, user: &User) -> Result<i32, UserRepositoryError> {
        let inserted_user_id = diesel::insert_into(schema::users::table)
            .values(user.clone())
            .returning(id)
            .get_result(&mut self.pool.get().unwrap())?;

        Ok(inserted_user_id)
    }

    async fn exists_by_email(&self, input_email: &str) -> Result<bool, UserRepositoryError> {
        let exists_by_email = select(exists(users.filter(email.eq(input_email))))
            .get_result(&mut self.pool.get().unwrap())?;

        Ok(exists_by_email)
    }

    async fn find_by_email(
        &self,
        input_email: String,
    ) -> Result<Option<User>, UserRepositoryError> {
        let user = users
            .filter(email.eq(input_email))
            .first::<User>(&mut self.pool.get().unwrap())
            .optional()?;

        Ok(user)
    }
}
