#![warn(clippy::pedantic)]
use axum::serve;
use config::{Config, Environment};
use hello_cargo::app_with_file_dir;
use std::path::Path;
use utoipa_scalar::{Scalar, Servable};

#[tokio::main]
async fn main() {
    if Path::new(".env").is_file() {
        dotenvy::dotenv().unwrap();
    }

    let settings = Config::builder()
        .set_default("host", "127.0.0.1")
        .unwrap()
        .set_default("port", 3000)
        .unwrap()
        .set_default("file_dir", "files")
        .unwrap()
        .add_source(
            Environment::default()
                .try_parsing(true)
                .convert_case(config::Case::Lower),
        )
        .build()
        .unwrap();
    let host = settings.get_string("host").unwrap();
    let port = settings.get::<u16>("port").unwrap();
    let file_dir = settings.get_string("file_dir").unwrap();

    let listener = tokio::net::TcpListener::bind((host.as_str(), port))
        .await
        .unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    let (app, doc) = app_with_file_dir(file_dir);
    let app = app.merge(Scalar::with_url("/docs", doc));
    serve(listener, app).await.unwrap();
}
