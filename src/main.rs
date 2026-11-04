#![warn(clippy::pedantic)]
use axum::serve;
use hello_cargo::app;
use std::net::SocketAddr;
use utoipa_scalar::{Scalar, Servable};
#[tokio::main]
async fn main() {
    let (app, doc): (axum::Router, utoipa::openapi::OpenApi) = app();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    let app = app.merge(Scalar::with_url("/docs", doc));
    serve(listener, app).await.unwrap();
}
