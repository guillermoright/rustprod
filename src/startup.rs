use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::routes::{health_check, subscribe};

pub fn run(
    listener: TcpListener,
    _pool: bb8::Pool<bb8_tiberius::ConnectionManager>,
) -> Result<Server, std::io::Error> {
    let pool = web::Data::new(_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
            .app_data(pool.clone())
        // Added subscribe route
    })
    .listen(listener)?
    .run();

    Ok(server)
}
