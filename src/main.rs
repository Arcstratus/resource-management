use actix_cors::Cors;
use actix_web::{get, middleware, App, HttpResponse, HttpServer, Responder};
use tracing::Level;
use tracing_subscriber::EnvFilter;
#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        .init();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://arcstratus.io")
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
            .max_age(3600)
            .supports_credentials();

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(ping)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
