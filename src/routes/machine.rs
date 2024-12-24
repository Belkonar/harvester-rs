use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use crate::models::AppState;

pub fn machine_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/status", get(crate::routes::main::status))
}
