use crate::models::AppState;
use crate::routes::machine::machine_routes;
use crate::routes::main::main_routes;
use axum::Router;

mod machine;
mod main;

pub fn collect_routes() -> Router<AppState> {
    Router::new().merge(main_routes()).merge(machine_routes())
}
