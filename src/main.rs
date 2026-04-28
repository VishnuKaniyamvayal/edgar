mod app_state;
mod routers;
mod handlers;


use app_state::AppState;
use sqlx::{ postgres::PgPoolOptions};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_connection_str = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new().max_connections(5).connect(&db_connection_str).await.expect("connection to database failed");

    let app_state = AppState {
        db: pool
    };

    let app = routers::init_router(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap()
}