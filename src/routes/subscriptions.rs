use actix_web::{web, Result};
use serde::Deserialize;
use tracing::Instrument;
use std::error::Error;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<bb8::Pool<bb8_tiberius::ConnectionManager>>,
) -> Result<String, Box<dyn Error>> {
    print!("{} {}", form.email, form.name);
    let request_span = tracing::info_span!( "Adding a new subscriber",
        subscriber_email = %form.email,
        subscriber_name= %form.name,
    );
    let _request_span_guard = request_span.enter();
        let query_span = tracing::info_span!(
    "Saving new subscriber details in the database"
        );
    let mut conn = pool.get().await?;
    let res = conn
        .simple_query("SELECT @@version")
        .instrument(query_span)
        .await?
        .into_first_result()
        .await?
        .into_iter()
        .map(|row| {
            let val: &str = row.get(0).unwrap();
            String::from(val)
        })
        .collect::<Vec<_>>();

    println!("{:?}", &res);
    Ok(format!("Welcome {}!", form.name))
}
