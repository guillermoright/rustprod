#[cfg(test)]
mod tests {
    use actix_web::web;
    use actix_web::{http::StatusCode, test, App};
    use zero2prod::health_check;

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
}
