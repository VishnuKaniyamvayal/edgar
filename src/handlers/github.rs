//listener for github webhook

use axum::Json;
use serde_json::Value;

pub async fn github_listener(Json(payload): Json<Value>) -> &'static str {
    println!("{:#?}", payload);
    "ok"
}