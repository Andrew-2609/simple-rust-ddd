use crate::presentation::routes;

use super::repositories::postgres_user_repository::PostgresUserRepository;
use actix_web::{App, HttpServer, middleware::Logger, web};
use log::info;

#[cfg(not(tarpaulin_include))]
pub async fn run() -> std::io::Result<()> {
    let repo = PostgresUserRepository::new();
    let app_data = web::Data::new(repo);

    info!("Starting...");

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(Logger::default())
            .configure(routes::user_routes::routes)
    })
    .bind("0.0.0.0:4000")
    .unwrap()
    .run()
    .await
}
