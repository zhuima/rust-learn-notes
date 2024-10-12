use crate::routes::{health_check, subscribe};
use sqlx::postgres::PgPool;
use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;


pub async fn run(listener: TcpListener, connection_pool: Pool) -> Result<Server, std::io::Error> {

    let connection = web::Data::new(connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}


