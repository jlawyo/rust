use axum::{
    routing::get,
    Router,
};

async fn hello_world() -> &'static str {
    "Hello, World!"
}
pub fn app() -> Router {
    Router::new().route("/hello", get(hello_world))
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