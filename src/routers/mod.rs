mod health;
mod webhook_listeners;

use axum::Router;
use crate::{app_state::AppState};

pub fn init_router(state: AppState) -> Router {
    Router::new()
        .merge(health::routes())
        .merge(webhook_listeners::github_router())
        .with_state(state)
}