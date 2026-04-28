use axum::{Router, routing::{ post }};

use crate::{app_state::AppState, handlers::github::github_listener};

pub fn github_router() -> Router<AppState> {
    Router::new().route("/github-wh", post(github_listener))
}