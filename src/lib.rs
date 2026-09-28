
use axum::{
    routing::get,
    Router,
};

async fn hello_world() -> &'static str {
    "Hello, World!"
}
pub fn app() -> Router {
    Router::new()
        .route("/hello", get(hello_world))
        .route("/echo", get(echo_text))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hello_world() {
        let response = hello_world().await;
        assert_eq!(response, "Hello, World!");
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