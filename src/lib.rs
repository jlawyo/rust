use axum::{routing::get, Json, Router};
use serde_json::json;
use axum_test::TestServer;

async fn hello_world() -> Json<serde_json::Value> {
    Json(json!({ "message": "Hello, World!" }))
}

pub fn app() -> Router {
    Router::new()
        .route("/hello", get(hello_world))
        .route("/echo", get(echo_text))
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
use axum::{
    extract::Query,
    Json,
};
use serde_derive::{Deserialize, Serialize};

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