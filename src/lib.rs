pub mod doc;
mod endpoints{

use axum::{Json};
use serde_json::json;
use axum::{
    extract::Query,
};
use serde_derive::{Deserialize, Serialize};


#[utoipa::path(get, path = "/hello")]
pub async fn hello_world() -> Json<serde_json::Value> {
    Json(json!({ "message": "Hello, World!" }))
}


#[derive(Debug, Deserialize, Serialize, utoipa::ToSchema)]
pub struct Echo {
    pub text: Option<String>,
}

#[utoipa::path(get, path = "/echo", 
        tag = "Echo",
        summary = "Echo Text Parameter",
        params(("text" = Option<String>, Query, description = "Text to echo")),
        description = "Returns the text sent in the query parameter. If no text is provided, it returns a default message.",
        responses(
            (status = 200, description = "Echo response", body = Echo),
            (status = 400, description = "Bad Request"),
        )
)]

pub async fn echo_text(Query(params): Query<Echo>) -> Json<Echo> {
    let Echo { text } = params;
    match text{
        Some(echoed_text) => Json(Echo { text: Some(echoed_text) }),
        None =>   Json(Echo { text: Some(String::from("Echo"))}),
    }
}


#[cfg(test)]
mod tests {

    use axum_test::{TestServer};
    use serde_json::json;

    #[tokio::test]
    async fn test_hello_world_json() {
        let (router, _) = crate::app();
        let server = TestServer::new(router);

        let response = server.get("/hello").await;

        response.assert_json(&json!({ "message": "Hello, World!" }));
    }
    #[tokio::test]
    async fn test_echo() {
        let (router, _) = crate::app();
        let server = TestServer::new(router);

        let response = server.get("/echo").await;

        response.assert_json(&json!({ "text": "Echo" }));
    }

}

}
use axum::Router;
use axum::{
    body::Bytes,
    extract::{Path as AxumPath, State},
    http::{header, StatusCode},
    routing::get,
};
use std::path::{Component, Path, PathBuf};
use tokio::fs;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::doc::ApiDoc;
pub fn app() -> (Router, utoipa::openapi::OpenApi) {
    let (router, doc) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(endpoints::hello_world))
        .routes(routes!(endpoints::echo_text))
        .split_for_parts();

    (router.merge(file_router("files")), doc)
}

#[derive(Clone)]
struct FileStore {
    directory: PathBuf,
}

fn file_router(directory: impl Into<PathBuf>) -> Router {
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