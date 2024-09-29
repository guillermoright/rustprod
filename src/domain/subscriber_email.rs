use std::borrow::Cow;

use validator::ValidateEmail;

#[derive(Debug, Clone)]
pub struct SubscriberEmail(String);

// Implement the `parse` method for `SubscriberEmail`.
impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        // Create a temporary SubscriberEmail for validation
        let temp_email = SubscriberEmail(s);

        // Use `validate_email` to check if it is a valid email.
        if temp_email.validate_email() {
            Ok(temp_email)
        } else {
            Err(format!("{} is not a valid subscriber email.", temp_email.0))
        }
    }
}

/// Implement `AsRef<str>` for `SubscriberEmail`.
impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl ValidateEmail for SubscriberEmail {
    fn as_email_string(&self) -> Option<Cow<str>> {
        Some(Cow::from(self.0.as_str()))
    }
}

// [...]
// #[cfg(test)]
// mod tests {
//     use super::SubscriberEmail;
//     use claim::assert_err;
//     #[test]
//     fn empty_string_is_rejected() {
//         let email = "".to_string();
//         assert_err!(SubscriberEmail::parse(email));
//     }
//     #[test]
//     fn email_missing_at_symbol_is_rejected() {
//         let email = "ursuladomain.com".to_string();
//         assert_err!(SubscriberEmail::parse(email));
//     }
//     #[test]
//     fn email_missing_subject_is_rejected() {
//         let email = "@domain.com".to_string();
//         assert_err!(SubscriberEmail::parse(email));
//     }
// }
