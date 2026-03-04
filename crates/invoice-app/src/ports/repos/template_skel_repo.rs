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
pub struct CreateTemplateSkel {
    pub name: String,
    pub company: CompanyId,
    pub client: ClientId,
    pub terms: TermsId,
    pub methods: Vec<MethodId>,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateTemplateSkel {
    pub name: Option<String>,
    pub company: Option<CompanyId>,
    pub client: Option<ClientId>,
    pub terms: Option<TermsId>,
    pub methods: Option<Vec<MethodId>>,
}

#[async_trait]
pub trait TemplateSkelRepo: Send + Sync {
    async fn get(&self, id: TemplateId) -> Result<Option<TemplateSkel>>;
    async fn list(&self) -> Result<Vec<TemplateSkel>>;

    async fn create(&self, input: CreateTemplateSkel) -> Result<TemplateId>;
    async fn update(&self, id: TemplateId, patch: UpdateTemplateSkel) -> Result<()>;
    async fn delete(&self, id: TemplateId) -> Result<bool>;
}


