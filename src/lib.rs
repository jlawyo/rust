use axum::{routing::get, Json, Router};
use serde_json::json;
use axum::{
    extract::Query,
};
use serde_derive::{Deserialize, Serialize};


async fn hello_world() -> Json<serde_json::Value> {
    Json(json!({ "message": "Hello, World!" }))
}

pub fn app() -> Router {
    Router::new()
        .route("/hello", get(hello_world))
        .route("/echo", get(echo_text))
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