use crate::schema::users::dsl::{email, users};
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
    async fn find_by_email(&self, input_email: String) -> Option<User> {
        users
            .filter(email.eq(input_email))
            .first::<User>(&mut self.pool.get().unwrap())
            .optional()
            .unwrap_or(None)
    }

    async fn save(&self, user: &User) -> Result<(), diesel::result::Error> {
        diesel::insert_into(schema::users::table)
            .values(user.clone())
            .execute(&mut self.pool.get().unwrap())
            .unwrap();

        Ok(())
    }
}
