use crate::domain::NewSubscriber;
use crate::domain::SubscriberEmail;
use crate::domain::SubscriberName;
use actix_web::http::StatusCode;
use actix_web::ResponseError;
use actix_web::{web, HttpResponse, Result};
use chrono::Utc;
use serde::Deserialize;
use std::error::Error;
use std::fmt;
use tracing::Instrument;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;
use validator::ValidateEmail;
use anyhow::Context;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[derive(thiserror::Error)]
pub enum SubscribeError {
    #[error("{0}")]
    ValidationError(String),
    // Transparent delegates both `Display`'s and `source`'s implementation
    // to the type wrapped by `UnexpectedError`.
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

// Implement `std::fmt::Debug` using the `error_chain_fmt` function for pretty printing
impl fmt::Debug for SubscribeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for SubscribeError {
    fn status_code(&self) -> StatusCode {
        match self {
            SubscribeError::ValidationError(_) => StatusCode::BAD_REQUEST,
            SubscribeError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// // Implement `std::error::Error` for `SubscribeError`
// impl std::error::Error for SubscribeError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         match self {
//             SubscribeError::ValidationError(_) => None,
//             SubscribeError::SendEmailError(e) => Some(e),
//         }
//     }
// }
// impl From<String> for SubscribeError {
//     fn from(e: String) -> Self {
//     Self::ValidationError(e)
//     }
//     }
// impl From<reqwest::Error> for SubscribeError {
//     fn from(e: reqwest::Error) -> Self {
//     Self::SendEmailError(e)
//     }
//     }

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
) -> Result<HttpResponse, SubscribeError> {
    print!("{} {}", form.email, form.name);

    // Convert the form data into a NewSubscriber struct
    let new_subscriber = form.0.try_into().map_err(SubscribeError::ValidationError)?;

    insert_subscriber(&pool, &new_subscriber)
        .await
        .context("Failed to insert new subscriber in the database.")?;

    // Return a success response
    Ok(HttpResponse::Ok().json(format!(
        "Subscriber {} successfully added!",
        &new_subscriber.name
    )))
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
) -> Result<String, anyhow::Error> {
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

// Helper function for formatting the error chain
fn error_chain_fmt(e: &impl std::error::Error, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    writeln!(f, "{}\n", e)?; // Print the main error message
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?; // Print each cause in the chain
        current = cause.source();
    }
    Ok(())
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
