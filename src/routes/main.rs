use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Json, Router};
use axum::routing::get;
use sqlx::Row;
use tower_http::cors::CorsLayer;
use crate::errors::{AppError, AppResult};
use crate::models::{AppState, Status};

pub async fn status(State(state): State<Arc<AppState>>) -> AppResult<Json<Status>> {
    let row = sqlx::query("SELECT 'hi' as text")
        .fetch_one(&state.db)
        .await
        .map_err(|_| AppError::status(StatusCode::NOT_FOUND))?;

    let text: &str = row.try_get("text").unwrap();

    Ok(Json(Status {
        status: text.to_string()
    }))
}

pub fn main_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(status))
        .layer(CorsLayer::permissive())
}