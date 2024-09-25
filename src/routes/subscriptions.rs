use actix_web::{web, Result};
use async_std::net::TcpStream;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>) -> Result<String> {
    Ok(format!("Welcome {}!", form.name))
}
