use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::env;

use std::net::TcpListener;

static CONN_STR_PORT: Lazy<String> = Lazy::new(|| {
    env::var("TIBERIUS_TEST_CONNECTION_STRING").unwrap_or_else(|_| {
        "server=tcp:127.0.0.1\\SQL2022D,23241;database=DestinationDB;IntegratedSecurity=true;TrustServerCertificate=true".to_owned()
    })
});

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[derive(Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(form: web::Form<FormData>) -> Result<String> {
    Ok(format!("Welcome {}!", form.name))
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe)) // Added subscribe route
    })
    .listen(listener)?
    .run();

    Ok(server)
}
