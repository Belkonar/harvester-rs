use crate::models::AppState;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;

pub fn machine_routes() -> Router<Arc<AppState>> {
    Router::new().route("/status", get(crate::routes::main::status))
}
