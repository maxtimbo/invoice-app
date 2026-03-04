use async_trait::async_trait;
use anyhow::Result;

use invoice_core::models::{
    client::Client,
    contact::Contact,
    ids::ClientId,
};

#[derive(Debug, Clone)]
pub struct CreateClient {
    pub name: String,
    pub contact: Contact,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateClient {
    pub name: Option<String>,
    pub contact: Option<Contact>,
}

#[async_trait]
pub trait ClientRepo: Send + Sync {
    async fn get(&self, id: ClientId) -> Result<Option<Client>>;
    async fn list(&self) -> Result<Vec<Client>>;

    async fn create(&self, input: CreateClient) -> Result<ClientId>;
    async fn update(&self, id: ClientId, patch: UpdateClient) -> Result<()>;
    async fn delete(&self, id: ClientId) -> Result<bool>;
}
