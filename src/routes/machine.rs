use crate::errors::AppResult;
use crate::models::AppState;
use axum::extract::State;
use axum::routing::{get, put};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;

#[derive(Deserialize)]
struct PutSourceRequest {
    id: String,
    name: String,
}

async fn put_source(
    State(state): State<Arc<AppState>>,
    Json(body): Json<PutSourceRequest>,
) -> AppResult<Json<Value>> {
    Ok(Json(json!({})))
}

pub fn machine_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/status", get(crate::routes::main::status))
        .route("/source", put(put_source))
}
