#[cfg(test)]
mod tests {
use super::*;
    use axum_test::{TestServer};
    use serde_json::json;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};
    use tokio::fs;

    static NEXT_TEST_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    fn file_test_server() -> (TestServer, PathBuf) {
        let directory = std::env::temp_dir()
            .join("hello_cargo_file_tests")
            .join(format!(
                "{}-{}",
                std::process::id(),
                NEXT_TEST_DIRECTORY.fetch_add(1, Ordering::Relaxed)
            ));
        (
            TestServer::new(file_router(directory.clone())),
            directory,
        )
    }

        #[tokio::test]
            async fn test_hello_world_json() {
                let (router, _) = app();
                let server = TestServer::new(router);
                let response = server.get("/hello").await;
                response.assert_json(&json!({ "message": "Hello, World!" }));
            }
        #[tokio::test]
        async fn test_echo() {
            let (router, _) = app();
            let server = TestServer::new(router);
            let response = server.get("/echo").await;
            response.assert_json(&json!({ "text": "Echo" }));
        }

    #[tokio::test]
    async fn test_put_file_then_get_contents() {
        let (server, directory) = file_test_server();

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
        let (server, directory) = file_test_server();

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