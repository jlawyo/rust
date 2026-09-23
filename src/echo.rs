use axum::{
    extract::Query,
    routing::get,
    Router,
    Json,
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Echo {
    pub text: String,
}

pub async fn echo_text(Query(params): Query<Echo>)-> Json<Echo> {
    let Echo { text } = params;
    let echoed_text = format!("Echo: {}", text);
    Json(Echo { text: echoed_text })
}
