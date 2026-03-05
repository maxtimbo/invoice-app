use async_trait::async_trait;
use anyhow::Result;

use invoice_core::models::{terms::Terms, ids::TermsId};

#[derive(Debug, Clone)]
pub struct CreateTerms {
    pub name: String,
    pub due: i64,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateTerms {
    pub name: Option<String>,
    pub due: Option<i64>,
}

#[async_trait]
pub trait TermsRepo: Send + Sync {
    async fn get_terms(&self, id: TermsId) -> Result<Option<Terms>>;
    async fn list_terms(&self) -> Result<Vec<Terms>>;

    async fn create_terms(&self, input: CreateTerms) -> Result<TermsId>;
    async fn update_terms(&self, id: TermsId, patch: UpdateTerms) -> Result<()>;

    async fn delete_terms(&self, id: TermsId) -> Result<bool>;
}
