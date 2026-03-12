use async_trait::async_trait;
use anyhow::Result;

use invoice_core::models::config::Config;

#[derive(Debug, Clone)]
pub struct UpsertConfig {
    pub smtp_server: String,
    pub port: u16,
    pub tls: bool,
    pub username: String,
    pub password: String,
    pub fromname: String,
    pub test_recipient: Option<String>,
}

#[async_trait]
pub trait ConfigRepo: Send + Sync {
    async fn get_config(&self) -> Result<Option<Config>>;
    async fn upsert_config(&self, input: UpsertConfig) -> Result<()>;
}
