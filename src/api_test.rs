use axum::http::StatusCode;
use reqwest::Client;
use std::net::SocketAddr;
use hello_cargo::app;

#[tokio::test]
async fn test_hello_endpoint() {
    let app = app();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    let server = axum::Server::bind(&addr).serve(app.into_make_service());
    tokio::spawn(server);

    let client = Client::new();
    let response = client.get("http://localhost:3000/hello").send().await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.text().await.unwrap();
    assert_eq!(body, "Hello, World!");
}  
