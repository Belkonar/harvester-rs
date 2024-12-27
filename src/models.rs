use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: PgPool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub status: String,
}

#[derive(Serialize, Debug)]
pub struct DataSource {
    pub id: sqlx::types::Uuid,
    pub name: String,
}
