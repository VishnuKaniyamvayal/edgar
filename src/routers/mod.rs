mod health;


use axum::Router;
use crate::app_state::AppState;

pub fn init_router(state: AppState) -> Router {
    Router::new()
        .merge(health::routes())
        .with_state(state)
}