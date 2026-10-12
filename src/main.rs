use std::net::SocketAddr;
use hello_cargo::app;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    let (app, doc): (axum::Router, utoipa::openapi::OpenApi) = app();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();


    let app = app.merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", doc));
    axum::serve(listener, app).await.unwrap();
}