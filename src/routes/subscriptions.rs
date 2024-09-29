use crate::domain::NewSubscriber;
use crate::domain::SubscriberName;
use actix_web::{web, HttpResponse, Result};
use chrono::Utc;
use serde::Deserialize;
use std::error::Error;
use tracing::Instrument;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
    request_id = %Uuid::new_v4(),
    subscriber_email = %form.email,
    subscriber_name= %form.name
    )
    )]
pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<bb8::Pool<bb8_tiberius::ConnectionManager>>,
) -> HttpResponse {
    print!("{} {}", form.email, form.name);

    let name: SubscriberName = match SubscriberName::parse(form.0.name) {
        Ok(name) => name,
        // Return early if the name is invalid, with a 400
        Err(_) => return HttpResponse::BadRequest().finish(),
        };
    
    if !is_valid_name(&name.as_str()) {
        return HttpResponse::BadRequest().finish();
    }
    
    let new_subscriber = NewSubscriber {
        email: form.0.email,
        name,
        };

    insert_subscriber(&pool, &new_subscriber)
        .await
        .expect("Failed to execute the query");
    HttpResponse::Ok().finish()
}

/// Returns `true` if the input satisfies all our validation constraints
/// on subscriber names, `false` otherwise.
pub fn is_valid_name(s: &str) -> bool {
    // `.trim()` returns a view over the input `s` without trailing
    // whitespace-like characters.
    // `.is_empty` checks if the view contains any character.
    let is_empty_or_whitespace = s.trim().is_empty();
    // A grapheme is defined by the Unicode standard as a "user-perceived"
    // character: `å` is a single grapheme, but it is composed of two characters
    // (`a` and `̊`).
    //
    // `graphemes` returns an iterator over the graphemes in the input `s`.
    // `true` specifies that we want to use the extended grapheme definition set,
    // the recommended one.
    let is_too_long = s.graphemes(true).count() > 256;
    // Iterate over all characters in the input `s` to check if any of them matches
    // one of the characters in the forbidden array.
    let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));
    // Return `false` if any of our conditions have been violated
    !(is_empty_or_whitespace || is_too_long || contains_forbidden_characters)
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, pool)
)]
pub async fn insert_subscriber(
    pool: &bb8::Pool<bb8_tiberius::ConnectionManager>,
    new_subscriber: &NewSubscriber,
) -> Result<String, Box<dyn Error>> {
    let mut conn = pool.get().await?;

    // Construct the query
    let query = r#"
        INSERT INTO LogEvents (message, level, exception, logevent)
        VALUES (@P1, @P2, @P3, @P4)
    "#;
    // Generate the dynamic values for the insert
    let id = Uuid::new_v4().to_string();
    let email = &new_subscriber.email;
    let name = new_subscriber.name.as_str();
    let subscribed_at = Utc::now().to_rfc3339();

    // Execute the query with dynamic parameters
    let _ = conn
        .execute(
            query,
            &[&id, email, &name, &subscribed_at], // Parameters as references
        )
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;

    Ok(format!("Welcome {}!", &new_subscriber.name))
}

// pub async fn select_subscriber(
//     pool: &bb8::Pool<bb8_tiberius::ConnectionManager>,
//     new_subscriber: &NewSubscriber,
// ) -> Result<String, Box<dyn Error>> {
//     let mut conn = pool.get().await?;

//     let res = conn
//         .simple_query("SELECT @@version")
//         .await?
//         .into_first_result()
//         .await?
//         .into_iter()
//         .map(|row| {
//             let val: &str = row.get(0).unwrap();
//             String::from(val)
//         })
//         .collect::<Vec<_>>();

//     println!("{:?}", &res);

//     Ok(format!("Welcome {}!", &new_subscriber.name))
// }
