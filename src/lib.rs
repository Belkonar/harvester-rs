use std::sync::Arc;
use axum::extract::State;
use axum::{Json, Router};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use serde_json::json;
use sqlx::{PgPool, Row};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use crate::models::AppState;

mod models;

struct AppError {
    code: StatusCode,
    message: String
}

impl AppError {
    pub fn status(code: StatusCode) -> AppError {
        AppError {
            code,
            message: "".to_string(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.code, self.message).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;

async fn status(State(state): State<Arc<AppState>>) -> AppResult<impl IntoResponse> {
    let row = sqlx::query("SELECT 'hi' as text")
        .fetch_one(&state.db)
        .await
        .map_err(|_| AppError::status(StatusCode::NOT_FOUND))?;

    let mut text: &str = row.try_get("text").unwrap();

    Ok(Json(json!({
        "status": text
    })))
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