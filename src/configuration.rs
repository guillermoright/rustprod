//! src/configuration.rs
#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}
#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub authentication: String,
    pub server_name: String,
    pub database_name: String,
    pub connection_string: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialise our configuration reader
    let settings = config::Config::builder()
        .set_default("default", "1")?
        .add_source(config::File::with_name("configuration"))
        .set_override("override", "1")?
        .build()
        .unwrap();
    settings.try_deserialize()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "Server={};Database={};Authentication={}",
            self.server_name, self.database_name, self.authentication
        )
    }
}
