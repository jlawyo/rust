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
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::doc::ApiDoc;
pub fn app() -> (Router, utoipa::openapi::OpenApi) {
    OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(endpoints::hello_world))
        .routes(routes!(endpoints::echo_text))
        .split_for_parts()
}