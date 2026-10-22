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
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_TEST_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    async fn file_test_server() -> (TestServer, PathBuf) {
        let directory = std::env::temp_dir()
            .join("hello_cargo_file_tests")
            .join(format!(
                "{}-{}",
                std::process::id(),
                NEXT_TEST_DIRECTORY.fetch_add(1, Ordering::Relaxed)
            ));
        fs::create_dir_all(&directory).await.unwrap();
        (
            TestServer::new(app_with_file_dir(directory.clone())),
            directory,
        )
    }

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

    #[tokio::test]
    async fn test_put_file_then_get_contents() {
        let (server, directory) = file_test_server().await;

        let response = server
            .put("/file/file.txt")
            .text("Hello, file!")
            .await;
        response.assert_status_ok();

        let response = server.get("/file/file.txt").await;
        response.assert_status_ok();
        response.assert_text("Hello, file!");
        assert_eq!(response.headers()["content-type"], "application/text");

        fs::remove_dir_all(directory).await.unwrap();
    }

    #[tokio::test]
    async fn test_put_file_is_idempotent_and_replaces_contents() {
        let (server, directory) = file_test_server().await;

        for _ in 0..2 {
            server
                .put("/file/file.txt")
                .text("same contents")
                .await
                .assert_status_ok();
        }
        server
            .put("/file/file.txt")
            .text("updated contents")
            .await
            .assert_status_ok();

        let response = server.get("/file/file.txt").await;
        response.assert_text("updated contents");
        assert_eq!(std::fs::read_dir(&directory).unwrap().count(), 1);

        fs::remove_dir_all(directory).await.unwrap();
    }

}