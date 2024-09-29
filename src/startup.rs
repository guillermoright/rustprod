use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::{
    email_client::EmailClient,
    routes::{health_check, subscribe},
};

pub fn run(
    listener: TcpListener,
    pool: bb8::Pool<bb8_tiberius::ConnectionManager>,
    email_client: EmailClient,
) -> Result<Server, std::io::Error> {
    let pool = web::Data::new(pool);
    let email_client = web::Data::new(email_client);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
            .app_data(pool.clone())
            .app_data(email_client.clone())
        // Added subscribe route
    })
    .listen(listener)?
    .run();

    Ok(server)
}
