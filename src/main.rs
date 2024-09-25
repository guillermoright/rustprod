use std::net::TcpListener;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address);
    let port = listener.as_ref().unwrap().local_addr().unwrap().port();
    println!("Server is running at {}", port);
    run(listener?)?.await
}
