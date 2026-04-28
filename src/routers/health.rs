use axum::{Router, routing::{ get }};

use crate::{app_state::AppState, handlers::health::check};

pub fn routes() -> Router<AppState> {
    Router::new().route("/health", get(check))
}