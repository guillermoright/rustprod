#[cfg(test)]
mod tests {
    use actix_web::web;
    use actix_web::{http::StatusCode, test, App};
    use async_std::net::TcpStream;
    use once_cell::sync::Lazy;
    use std::error::Error;
    use std::{env, net::TcpListener};
    use tiberius::{AuthMethod, Client, Config};
    use zero2prod::configuration::get_configuration;
    use zero2prod::routes::health_check;
    use zero2prod::startup::run;

    static CONN_STR: Lazy<String> = Lazy::new(|| {
        env::var("TIBERIUS_TEST_CONNECTION_STRING").unwrap_or_else(|_| {
            "Server=127.0.0.1\\mssqlserver19,1443;Database=AuthLogs;User Id=SA;Password=P4ssw0rd!;Encrypt=True;TrustServerCertificate=True;".to_owned()
        })
    });

    #[actix_web::test]
    async fn test_health_check() {
        // Create a test server instance of your app
        let app = test::init_service(App::new().route("/", web::get().to(health_check))).await;

        // Create a request to the "/" route
        let req = test::TestRequest::get().uri("/").to_request();

        // Send the request and get the response
        let resp = test::call_service(&app, req).await;

        // Check that the response has a 200 OK status
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn subscribe_returns_a_200_for_valid_form_data() {
        // Create a test server instance of your app
        // let config = Config::from_ado_string(&CONN_STR)?;

        // let tcp = TcpStream::connect(config.get_addr()).await.map_err(|e| Box::new(e) as Box<dyn Error>)?;
        // tcp.set_nodelay(true)?;

        // let mut client = Client::connect(config, tcp).await?;
        // let stream = client.query("SELECT @P1", &[&1i32]).await?;
        // let row = stream.into_row().await?.unwrap();

        // // Check that the response has a 200 OK status
        // assert_eq!(resp.status(), StatusCode::OK);
    }
}
