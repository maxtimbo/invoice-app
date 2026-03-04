use async_trait::async_trait;
use anyhow::Result;

use invoice_core::models::{
    company::Company,
    contact::Contact,
    ids::CompanyId,
};

#[derive(Debug, Clone)]
pub struct CreateCompany {
    pub name: String,
    pub logo: Option<Vec<u8>>,
    pub contact: Contact,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateCompany {
    pub name: Option<String>,
    pub logo: Option<Vec<u8>>,
    pub contact: Option<Contact>,
}

#[async_trait]
pub trait CompanyRepo: Send + Sync {
    async fn get(&self, id: CompanyId) -> Result<Option<Company>>;
    async fn list(&self) -> Result<Vec<Company>>;

    async fn create(&self, input: CreateCompany) -> Result<CompanyId>;
    async fn update(&self, id: CompanyId, patch: UpdateCompany) -> Result<()>;
    async fn delete(&self, id: CompanyId) -> Result<bool>;
}
