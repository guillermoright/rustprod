use std::{env, net::TcpListener};
use async_std::net::TcpStream;
use once_cell::sync::Lazy;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use tiberius::{Client, Config, AuthMethod};
use std::error::Error;

static CONN_STR: Lazy<String> = Lazy::new(|| {
    env::var("TIBERIUS_TEST_CONNECTION_STRING").unwrap_or_else(|_| {
        "Server=127.0.0.1\\mssqlserver19,1443;Database=AuthLogs;User Id=SA;Password=P4ssw0rd!;Encrypt=True;TrustServerCertificate=True;".to_owned()
    })
});

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).map_err(|e| Box::new(e) as Box<dyn Error>)?;
    println!("Server is running at {}", configuration.application_port);

    let config = Config::from_ado_string(&CONN_STR)?;

    let tcp = TcpStream::connect(config.get_addr()).await.map_err(|e| Box::new(e) as Box<dyn Error>)?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp).await?;
    
    let stream = client.query("SELECT @P1", &[&1i32]).await?;
    let row = stream.into_row().await?.unwrap();
    println!("{:?}", row);
    run(listener)?.await.map_err(|e| Box::new(e) as Box<dyn Error>)?;
    Ok(())
}
