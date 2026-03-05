use sqlx::SqlitePool;
use anyhow::Result;

mod repos;
mod models;

#[derive(Clone)]
pub struct SqliteStorage {
    pool: SqlitePool,
}

impl SqliteStorage {
    pub async fn connect(database: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database).await?;
        Ok(Self { pool })
    }
}
