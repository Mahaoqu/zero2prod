use actix_web::dev::Server;
use actix_web::HttpResponse;
use actix_web::{get, web, App, HttpServer, Responder};
use std::net::TcpListener;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .route("/health_check", web::get().to(health_check))
            .service(greet)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
