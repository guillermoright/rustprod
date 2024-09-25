use actix_web::{web, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>) -> Result<String> {
    Ok(format!("Welcome {}!", form.name))
}
