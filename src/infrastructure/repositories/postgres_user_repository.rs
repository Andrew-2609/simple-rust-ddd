use crate::schema::users::dsl::{email, id, users};
use crate::{
    domain::{entities::user::User, repositories::user_repository::UserRepository},
    infrastructure::db::connection::{DBPool, establish_connection},
    schema,
};
use async_trait::async_trait;
use diesel::prelude::*;
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

#[async_trait]
impl UserRepository for Arc<PostgresUserRepository> {
    async fn save(&self, user: &User) -> Result<i32, String> {
        diesel::insert_into(schema::users::table)
            .values(user.clone())
            .returning(id)
            .get_result(&mut self.pool.get().unwrap())
            .map_err(|err| err.to_string())
    }

    async fn find_by_email(&self, input_email: String) -> Result<Option<User>, String> {
        users
            .filter(email.eq(input_email))
            .first::<User>(&mut self.pool.get().unwrap())
            .optional()
            .map_err(|err| err.to_string())
    }
}
