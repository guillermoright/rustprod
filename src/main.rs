use std::net::TcpListener;

use zero2prod::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080");
    let port = listener.as_ref().unwrap().local_addr().unwrap().port();
    println!("Server is running at {}", port);
    run(listener?)?.await
}
