use async_trait::async_trait;
use anyhow::Result;

use invoice_core::models::{method::Method, ids::MethodId};

#[derive(Debug, Clone)]
pub struct CreateMethod {
    pub name: String,
    pub link: Option<String>,
    pub qr: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateMethod {
    pub name: Option<String>,
    pub link: Option<String>,
    pub qr: Option<Vec<u8>>,
}

#[async_trait]
pub trait MethodRepo: Send + Sync {
    async fn get_method(&self, id: MethodId) -> Result<Option<Method>>;
    async fn list_method(&self) -> Result<Vec<Method>>;

    async fn create_method(&self, input: CreateMethod) -> Result<MethodId>;
    async fn update_method(&self, id: MethodId, patch: UpdateMethod) -> Result<()>;
    async fn delete_method(&self, id: MethodId) -> Result<bool>;
}
