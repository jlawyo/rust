use axum::{routing::get, Json, Router};
use serde_json::json;
use axum::{
    body::Bytes,
    extract::{Path as AxumPath, State},
    http::{header, StatusCode},
};
use axum::{
    extract::Query,
};
use serde_derive::{Deserialize, Serialize};
use std::path::{Component, Path, PathBuf};
use tokio::fs;

#[derive(Clone)]
struct FileStore {
    directory: PathBuf,
}

async fn hello_world() -> Json<serde_json::Value> {
    Json(json!({ "message": "Hello, World!" }))
}

pub fn app() -> Router {
    Router::new()
        .route("/hello", get(hello_world))
        .route("/echo", get(echo_text))
        .merge(app_with_file_dir("files"))
}

fn app_with_file_dir(directory: impl Into<PathBuf>) -> Router {
    Router::new()
        .route("/file/{filename}", get(get_file).put(put_file))
        .with_state(FileStore {
            directory: directory.into(),
        })
}

fn file_path(directory: &Path, filename: &str) -> Result<PathBuf, StatusCode> {
    let path = Path::new(filename);
    if filename.contains('/')
        || filename.contains('\\')
        || !matches!(path.components().next(), Some(Component::Normal(_)))
        || path.components().count() != 1
    {
        return Err(StatusCode::BAD_REQUEST);
    }

    Ok(directory.join(path))
}

async fn put_file(
    State(store): State<FileStore>,
    AxumPath(filename): AxumPath<String>,
    contents: Bytes,
) -> Result<StatusCode, StatusCode> {
    let path = file_path(&store.directory, &filename)?;
    fs::create_dir_all(&store.directory)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    fs::write(path, contents)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

async fn get_file(
    State(store): State<FileStore>,
    AxumPath(filename): AxumPath<String>,
) -> Result<impl axum::response::IntoResponse, StatusCode> {
    let path = file_path(&store.directory, &filename)?;
    let contents = fs::read(path).await.map_err(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            StatusCode::NOT_FOUND
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        }
    })?;

    Ok(([(header::CONTENT_TYPE, "application/text")], contents))
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Echo {
    pub text: Option<String>,
}


pub async fn echo_text(Query(params): Query<Echo>) -> Json<Echo> {
    let Echo { text } = params;
    match text{
        Some(echoed_text) => Json(Echo { text: Some(echoed_text) }),
        None =>   Json(Echo { text: Some(String::from("Echo"))}),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::{TestResponse, TestServer};
    use serde_json::json;

    #[tokio::test]
    async fn test_hello_world_json() {
        let server = TestServer::new(app());

        let response = server.get("/hello").await;

        response.assert_json(&json!({ "message": "Hello, World!" }));
    }
    #[tokio::test]
    async fn test_echo() {
        let server = TestServer::new(app());

        let response = server.get("/echo").await;

        response.assert_json(&json!({ "text": "Echo" }));
    }

}