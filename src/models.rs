use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub struct AppState {
    pub db: PgPool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub status: String
}
