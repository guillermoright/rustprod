use std::error::Error;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::email_client::EmailClient;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("0.0.0.0:{}", configuration.application_port);
    let listener = TcpListener::bind(address).map_err(|e| Box::new(e) as Box<dyn Error>)?;
    println!("Server is running at {}", configuration.application_port);
    let cong = tiberius::Config::new();
    let mgr =
        bb8_tiberius::ConnectionManager::build(configuration.database.connection_string.as_str())?;

    let pool: bb8::Pool<bb8_tiberius::ConnectionManager> = bb8::Pool::builder().build(mgr).await?;

    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);

    // Build an `EmailClient` using `configuration`
    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address.");
    let timeout = configuration.email_client.timeout();
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
        timeout,
    );

    run(listener, pool, email_client)?
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
    Ok(())
}
