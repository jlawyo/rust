#![warn(clippy::pedantic)]
use axum::serve;
use config::{Config, Environment};
use hello_cargo::app_with_file_dir;
use std::path::Path;
use utoipa_scalar::{Scalar, Servable};

#[derive(serde_derive::Deserialize)]
struct Settings {
    host: String,
    port: u16,
    file_dir: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(".env").is_file() {
        dotenvy::dotenv()?;
    }

    let settings: Settings = Config::builder()
        .set_default("host", "127.0.0.1")?
        .set_default("port", 3000)?
        .set_default("file_dir", "files")?
        .add_source(
            Environment::default()
                .try_parsing(true)
                .convert_case(config::Case::Lower),
        )
        .build()?
        .try_deserialize()?;

    let (app, doc) = app_with_file_dir(settings.file_dir);
    let app = app.merge(Scalar::with_url("/docs", doc));
    let listener = tokio::net::TcpListener::bind((settings.host.as_str(), settings.port)).await?;
    println!("Listening on {}", listener.local_addr()?);
    serve(listener, app).await?;
    Ok(())
}
