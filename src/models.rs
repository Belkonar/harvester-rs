use serde::Serialize;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

#[derive(Serialize)]
pub struct Status {
    pub status: String,
}

#[derive(Serialize)]
pub struct DataSource {
    pub id: sqlx::types::Uuid,
    pub name: String,
}
