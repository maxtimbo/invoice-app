use async_trait::async_trait;
use anyhow::Result;

use invoice_core::models::{
    template_skel::TemplateSkel,
    ids::{
        TemplateId,
        CompanyId,
        ClientId,
        TermsId,
        MethodId,
    }
};

#[derive(Debug, Clone)]
pub struct CreateTemplate {
    pub name: String,
    pub company: CompanyId,
    pub client: ClientId,
    pub terms: TermsId,
    pub method: Vec<MethodId>,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateTemplate {
    pub name: Option<String>,
    pub company: Option<CompanyId>,
    pub client: Option<ClientId>,
    pub terms: Option<TermsId>,
    pub method: Option<Vec<MethodId>>,
}

#[async_trait]
pub trait TemplateRepo: Send + Sync {
    async fn get(&self, id: TemplateId) -> Result<Option<TemplateSkel>>;
    async fn list(&self) -> Result<Vec<TemplateSkel>>;

    async fn create(&self, input: CreateTemplate) -> Result<TemplateId>;
    async fn update(&self, id: TemplateId, patch: UpdateTemplate) -> Result<()>;
    async fn delete(&self, id: TemplateId) -> Result<bool>;
}


