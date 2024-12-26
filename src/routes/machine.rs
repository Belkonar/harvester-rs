use crate::errors::AppResult;
use crate::models::{AppState, DataSource};
use axum::extract::State;
use axum::routing::{get, put};
use axum::{Json, Router};
use serde::Deserialize;

#[derive(Deserialize)]
struct PutSourceRequest {
    id: String,
    name: String,
}

async fn put_source(
    State(_state): State<AppState>,
    Json(body): Json<PutSourceRequest>,
) -> AppResult<Json<DataSource>> {
    Ok(Json(DataSource {
        id: body.id,
        name: body.name,
    }))
}

pub fn machine_routes() -> Router<AppState> {
    Router::new()
        .route("/status", get(crate::routes::main::status))
        .route("/source", put(put_source))
}
