use actix_web::{HttpResponse, Responder};
use once_cell::sync::Lazy;
use std::env;

static CONN_STR_PORT: Lazy<String> = Lazy::new(|| {
    env::var("TIBERIUS_TEST_CONNECTION_STRING").unwrap_or_else(|_| {
        "server=tcp:127.0.0.1\\SQL2022D,23241;database=DestinationDB;IntegratedSecurity=true;TrustServerCertificate=true".to_owned()
    })
});

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
