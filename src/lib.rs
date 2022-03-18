use actix_web::HttpResponse;
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .route("/health_check", web::get().to(health_check))
            .service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
