use crate::errors::{AppError, AppResult};
use crate::models::{AppState, DataSource};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::put;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::types::Uuid;

#[derive(Deserialize)]
struct PutSourceRequest {
    id: Uuid,
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct PutTableRequest {
    source: Uuid,
    nonce: Uuid,
    names: Vec<String>,
}

async fn put_source(
    State(state): State<AppState>,
    Json(body): Json<PutSourceRequest>,
) -> AppResult<Json<DataSource>> {
    sqlx::query("CALL upsert_source($1, $2)")
        .bind(body.id)
        .bind(&body.name)
        .execute(&state.db)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(DataSource {
        id: body.id,
        name: body.name,
    }))
}

async fn put_tables(
    State(state): State<AppState>,
    Json(body): Json<PutTableRequest>,
) -> AppResult<Json<Value>> {
    let len = body.names.len();

    if len > 50 {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            format!(
                "This endpoint is limited to 50 items per request, you have sent {}",
                len
            ),
        ));
    }

    for table in &body.names {
        sqlx::query("CALL upsert_table($1, $2, $3)")
            .bind(table)
            .bind(body.source)
            .bind(body.nonce)
            .execute(&state.db)
            .await
            .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    Ok(Json(json!({})))
}

pub fn machine_routes() -> Router<AppState> {
    Router::new()
        .route("/source", put(put_source))
        .route("/tables", put(put_tables))
}
