use crate::models::AppState;
use crate::routes::collect_routes;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

mod models;
mod errors;
mod routes;

pub async fn setup() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:garden@localhost/harvester")
        .await
        .unwrap();

    let shared_state = Arc::new(AppState {
        db: pool
    });

    let app = collect_routes()
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}