use crate::domain::{NewSubscriber, SubscriberName};
use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;
use std::error::Error;
use tracing::Instrument;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<bb8::Pool<bb8_tiberius::ConnectionManager>>,
) -> HttpResponse {
    print!("{} {}", form.email, form.name);
    let new_subscriber = NewSubscriber {
        email: form.0.email,
        name: SubscriberName::parse(form.0.name),
    };
    if !is_valid_name(&new_subscriber.name.as_str()) {
        return HttpResponse::BadRequest().finish();
    }
    let request_span = tracing::info_span!( "Adding a new subscriber",
        subscriber_email = new_subscriber.email.as_str(),
        subscriber_name= new_subscriber.name.as_str(),
    );
    let _request_span_guard = request_span.enter();
    let query_span = tracing::info_span!("Saving new subscriber details in the database");

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

pub async fn insert_subscriber(
    pool: &bb8::Pool<bb8_tiberius::ConnectionManager>,
    new_subscriber: &NewSubscriber,
) -> Result<String, Box<dyn Error>> {

    let mut conn = pool.get().await?;

    let res = conn
        .simple_query("SELECT @@version")
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

        Ok(format!("Welcome {}!", &new_subscriber.name))

}