use std::sync::Arc;
use axum::Router;
use crate::models::AppState;
use crate::routes::machine::machine_routes;
use crate::routes::main::main_routes;

mod main;
mod machine;

pub fn collect_routes() -> Router<Arc<AppState>> {
    Router::new()
        .merge(main_routes())
        .merge(machine_routes())
}