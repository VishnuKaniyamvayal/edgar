use axum::Json;

pub async fn check() -> Json<&'static str> {
    Json("ok")
}