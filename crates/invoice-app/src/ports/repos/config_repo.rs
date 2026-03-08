use async_trait::async_trait;
use anyhow::Result;

use invoice_core::models::config::Config;

#[derive(Debug, Clone)]
pub struct CreateConfig {
    pub smtp_server: String,
    pub port: u16,
    pub tls: bool,
    pub username: String,
    pub password: String,
    pub fromname: String,
}

#[derive(Debugm Clone, Default)]
pub struct UpdateConfig {
    pub smtp_server: Option<String>,
    pub port: Option<u16>,
    pub tls: Option<bool>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub fromname: Option<String>,
}

#[async_trait]
pub trait ConfigRepo: Send + Sync {
    async fn get_config(&self) -> Result<Option<Config>>;
    async fn create_config(&self, input: CreateConfig) -> Result<()>;
    async fn update_config(&self, patch: UpdateConfig) -> Result<()>;
}
