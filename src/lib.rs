use axum::{routing::get, Json, Router};
use serde_json::json;
use axum::{
    extract::Query,
};
use serde_derive::{Deserialize, Serialize};
use axum::body::{Body, Bytes};
use axum::extract::{Path, State};
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::Response;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

type FileStore = Arc<RwLock<HashMap<String, Bytes>>>;

async fn hello_world() -> Json<serde_json::Value> {
    Json(json!({ "message": "Hello, World!" }))
}

pub fn app() -> Router {
    let files = HashMap::from([(
        "mock.txt".to_owned(),
        Bytes::from_static(b"This is a mock file."),
    )]);

    Router::new()
        .route("/hello", get(hello_world))
        .route("/echo", get(echo_text))
        .route("/file/{filename}", get(get_file).put(put_file))
        .with_state(Arc::new(RwLock::new(files)))
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

async fn get_file(
    Path(filename): Path<String>,
    State(files): State<FileStore>,
) -> Result<Response, StatusCode> {
    let contents = {
        let files = files.read().await;
        files.get(&filename).cloned()
    }
    .ok_or(StatusCode::NOT_FOUND)?;

    let mut response = Response::new(Body::from(contents));
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/text"),
    );
    Ok(response)
}

async fn put_file(
    Path(filename): Path<String>,
    State(files): State<FileStore>,
    contents: Bytes,
) -> StatusCode {
    files.write().await.insert(filename, contents);
    StatusCode::NO_CONTENT
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

    #[tokio::test]
    async fn test_get_file() {
        let server = TestServer::new(app());

        let response = server.get("/file/mock.txt").await;
        response.assert_text("This is a mock file.");
        response.assert_header("content-type", "application/text");
    }

    #[tokio::test]
    async fn test_put_then_get_file() {
        let server = TestServer::new(app());

        server
            .put("/file/notes.txt")
            .content_type("application/text")
            .text("Updated file.")
            .await
            .assert_status(StatusCode::NO_CONTENT);

        let response = server.get("/file/notes.txt").await;
        response.assert_text("Updated file.");
        response.assert_header("application/text");
    }

}