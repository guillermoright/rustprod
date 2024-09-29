use crate::domain::NewSubscriber;
use crate::domain::SubscriberEmail;
use crate::domain::SubscriberName;
use actix_web::{web, HttpResponse, Result};
use chrono::Utc;
use serde::Deserialize;
use std::error::Error;
use tracing::Instrument;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;
use validator::ValidateEmail;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub fn parse_subscriber(form: FormData) -> Result<NewSubscriber, String> {
    let name = SubscriberName::parse(form.name)?;
    let email = SubscriberEmail::parse(form.email)?;
    Ok(NewSubscriber { email, name })
}

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;
    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;
        Ok(Self { email, name })
    }
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

    let new_subscriber = match form.0.try_into() {
        Ok(form) => form,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    match insert_subscriber(&pool, &new_subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
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
    let email = new_subscriber.email.as_email_string();
    let name = new_subscriber.name.as_str();
    let subscribed_at = Utc::now().to_rfc3339();

    // Execute the query with dynamic parameters
    let _ = conn
        .execute(
            query,
            &[&id, &email, &name, &subscribed_at], // Parameters as references
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
