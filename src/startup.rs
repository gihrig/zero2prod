//! src/startup.rs
use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
// use std::sync::Arc;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap the pool using web::Data, which boils down to Arc smart pointer
    let db_pool = web::Data::new(db_pool);
    // Capture `db_pool` from the surrounding environment
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // Our pool is already wrapped in a `Data:`
            // using .data would add another Arc pointer on top
            // of the existing one - an unnecessary indirection.
            // .app_data instead does not perform an additional layer
            // of wrapping.
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
