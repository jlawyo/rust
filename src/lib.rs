use axum::{routing::get, Json, Router};
use serde_json::json;
use axum_test::TestServer;

async fn hello_world() -> Json<serde_json::Value> {
    Json(json!({ "message": "Hello, World!" }))
}

pub fn app() -> Router {
    Router::new().route("/hello", get(hello_world))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;
    use serde_json::json;

    #[tokio::test]
    async fn test_hello_world_json() {
        let server = TestServer::new(app());

        let response = server.get("/hello").await;

        response.assert_json(&json!({ "message": "Hello, World!" }));
    }
}