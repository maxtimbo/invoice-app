use async_trait::async_trait;
use anyhow::Result;

use invoice_core::models::{item::Item, ids::ItemId, currency::Currency};

#[derive(Debug, Clone)]
pub struct CreateItem {
    pub name: String,
    pub rate: Currency,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateItem {
    pub name: Option<String>,
    pub rate: Option<Currency>,
}

#[async_trait]
pub trait ItemRepo: Send + Sync {
    async fn get(&self, id: ItemId) -> Result<Option<Item>>;
    async fn list(&self) -> Result<Vec<Item>>;

    async fn create(&self, input: CreateItem) -> Result<ItemId>;
    async fn update(&self, id: ItemId, patch: UpdateItem) -> Result<()>;

    async fn delete(&self, id: ItemId) -> Result<bool>;
}
