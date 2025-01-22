use crate::errors::{json_ok, AppError, AppResult, JsonResult};
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

#[derive(Deserialize)]
struct PutTableRequest {
    source: Uuid,
    nonce: Uuid,
    names: Vec<String>,
}

#[derive(Deserialize)]
struct PutFieldRequest {
    source: Uuid,
    nonce: Uuid,
    fields: Vec<PutFieldRequestField>,
}

#[derive(Deserialize)]
struct PutFieldRequestField {
    name: String,
    table: String,
    types: Vec<String>,
    subfield: bool,
}

async fn put_source(
    State(state): State<AppState>,
    Json(body): Json<PutSourceRequest>,
) -> JsonResult<DataSource> {
    sqlx::query("CALL upsert_source($1, $2)")
        .bind(body.id)
        .bind(&body.name)
        .execute(&state.db)
        .await
        .map_err(AppError::error_from)?;

    json_ok(DataSource {
        id: body.id,
        name: body.name,
    })
}

async fn put_tables(
    State(state): State<AppState>,
    Json(body): Json<PutTableRequest>,
) -> JsonResult<Value> {
    let len = body.names.len();

    if len > 50 {
        return Err(AppError::bad_request(format!(
            "This endpoint is limited to 50 items per request, you have sent {}",
            len
        )));
    }

    for table in &body.names {
        sqlx::query("CALL upsert_table($1, $2, $3)")
            .bind(table)
            .bind(body.source)
            .bind(body.nonce)
            .execute(&state.db)
            .await
            .map_err(AppError::error_from)?;
    }

    json_ok(json!({}))
}

async fn put_fields(
    State(state): State<AppState>,
    Json(body): Json<PutFieldRequest>,
) -> JsonResult<Value> {
    let len = body.fields.len();

    if len > 500 {
        return Err(AppError::bad_request(format!(
            "This endpoint is limited to 50 items per request, you have sent {}",
            len
        )));
    }

    for field in &body.fields {
        sqlx::query("CALL upsert_field($1, $2, $3, $4, $5, $6)")
            .bind(&field.name)
            .bind(body.source)
            .bind(&field.table)
            .bind(&field.types)
            .bind(field.subfield)
            .bind(body.nonce)
            .execute(&state.db)
            .await
            .map_err(AppError::error_from)?;
    }

    json_ok(json!({}))
}

pub fn machine_routes() -> Router<AppState> {
    Router::new()
        .route("/source", put(put_source))
        .route("/tables", put(put_tables))
        .route("/fields", put(put_fields))
}
