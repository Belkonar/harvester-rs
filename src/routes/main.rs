use crate::errors::{AppError, AppResult, JsonResult};
use crate::models::{AppState, Status};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use sqlx::Row;
use tower_http::cors::CorsLayer;

pub async fn status(State(state): State<AppState>) -> JsonResult<Status> {
    let row = sqlx::query("SELECT 'hi' as text")
        .fetch_one(&state.db)
        .await
        .map_err(|_| AppError::not_found())?;

    let text: &str = row.try_get("text").unwrap();

    Ok(Json(Status {
        status: text.to_string(),
    }))
}

pub fn main_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(status))
        .layer(CorsLayer::permissive())
}
