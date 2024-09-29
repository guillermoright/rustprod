use secrecy::{ExposeSecret, SecretBox};

use crate::domain::SubscriberEmail;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
    pub email_client: EmailClientSettings,
}
#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretBox<String>,
    pub port: u16,
    pub authentication: String,
    pub server_name: String,
    pub database_name: String,
    pub connection_string: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let run_mode = std::env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("config");

    // Initialise our configuration reader
    let settings = config::Config::builder()
        .set_default("default", "1")?
        .add_source(config::File::from(configuration_directory.join("base")).required(true))
        .add_source(
            config::File::from(configuration_directory.join(run_mode.as_str())).required(true),
        )
        .set_override("override", "1")?
        .build()
        .unwrap();

    settings.try_deserialize()
}

impl DatabaseSettings {
    pub fn getconnection_string(&self) -> SecretBox<String> {
        SecretBox::new(Box::new(format!(
            "Server={};Database={};Authentication={}",
            self.server_name,
            self.database_name,
            self.password.expose_secret()
        )))
    }
}

#[derive(serde::Deserialize)]
pub struct EmailClientSettings {
    pub base_url: String,
    pub sender_email: String,
    pub authorization_token: SecretBox<String>,
    pub timeout_milliseconds: u64,
}
impl EmailClientSettings {
    pub fn sender(&self) -> Result<SubscriberEmail, String> {
        SubscriberEmail::parse(self.sender_email.clone())
    }
    pub fn timeout(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.timeout_milliseconds)
    }
}

// pub enum Environment {
//     Local,
//     Production,
// }

// impl Environment {
//     pub fn as_str(&self) -> &'static str {
//         match self {
//             Environment::Local => "local",
//             Environment::Production => "production",
//         }
//     }
// }

// impl TryFrom<String> for Environment {
//     type Error = String;
//     fn try_from(s: String) -> Result<Self, Self::Error> {
//         match s.to_lowercase().as_str() {
//             "local" => Ok(Self::Local),
//             "production" => Ok(Self::Production),
//             other => Err(format!(
//                 "{} is not a supported environment. Use either `local` or `production`.",
//                 other
//             )),
//         }
//     }
// }
