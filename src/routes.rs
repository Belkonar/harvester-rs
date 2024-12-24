use crate::models::AppState;
use crate::routes::machine::machine_routes;
use crate::routes::main::main_routes;
use axum::Router;
use std::sync::Arc;

mod machine;
mod main;

pub fn collect_routes() -> Router<Arc<AppState>> {
    Router::new().merge(main_routes()).merge(machine_routes())
}
