use invoice_storage::sqlite::SqliteStorage;
use invoice_app::commands::paths::Paths;
use std::sync::Arc;
use tokio::sync::Mutex;
use anyhow::{anyhow, Result};

pub struct AppState {
    pub db: Arc<Mutex<SqliteStorage>>,
}

impl AppState {
    pub async fn init() -> Result<Self> {
        let paths = Paths::init()?;
        let db = paths.db.to_str().ok_or_else(|| anyhow!("invalid db path"))?;
        Ok(Self {
            db: Arc::new(Mutex::new(SqliteStorage::connect(db).await?))
        })
    }
}
