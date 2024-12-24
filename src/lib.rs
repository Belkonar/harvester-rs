use crate::models::{AppState, Status};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use crate::errors::{AppError, AppResult};

mod models;
mod errors;

async fn status(State(state): State<Arc<AppState>>) -> AppResult<Json<Status>> {
    let row = sqlx::query("SELECT 'hi' as text")
        .fetch_one(&state.db)
        .await
        .map_err(|_| AppError::status(StatusCode::NOT_FOUND))?;

    let text: &str = row.try_get("text").unwrap();

    Ok(Json(Status {
        status: text.to_string()
    }))
}

pub async fn setup() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:garden@localhost/harvester")
        .await
        .unwrap();

    let shared_state = Arc::new(AppState {
        db: pool
    });

    let main_routes = Router::new()
        .route("/", get(status))
        .layer(CorsLayer::permissive());

    let machine_routes = Router::new()
        .route("/status", get(status));

    // build our application with a single route
    let app = Router::new()
        .merge(main_routes)
        .merge(machine_routes)
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}