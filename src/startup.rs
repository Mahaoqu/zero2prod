use crate::routes::*;

use actix_web::dev::Server;
use actix_web::{get, web, App, HttpServer, Responder};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
