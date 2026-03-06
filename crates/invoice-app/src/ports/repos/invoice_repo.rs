use async_trait::async_trait;
use anyhow::Result;
use chrono::NaiveDate;

use invoice_core::models::{
    ids::{InvoiceId, ItemId, TemplateId},
    invoice::Invoice,
    summary::InvoiceSummary,
    attributes::InvoiceAttrs,
    quantity::Quantity,
};

#[derive(Debug, Clone)]
pub struct CreateInvoice {
    pub template: TemplateId,
    pub date: NaiveDate,
    pub attributes: InvoiceAttrs,
    pub notes: Option<String>,
    pub items: Vec<(ItemId, Quantity)>,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateInvoice {
    pub show_methods: Option<bool>,
    pub show_notes: Option<bool>,
    pub stage: Option<invoice_core::models::stage::InvoiceStage>,
    pub status: Option<invoice_core::models::status::PaidStatus>,
    pub notes: Option<String>,
}

#[async_trait]
pub trait InvoiceRepo: Send + Sync {
    async fn get_invoice(&self, id: InvoiceId) -> Result<Option<Invoice>>;
    async fn list_invoice_summary(&self) -> Result<Vec<InvoiceSummary>>;
    async fn create_invoice(&self, input: CreateInvoice) -> Result<InvoiceId>;
    async fn update_invoice(&self, id: InvoiceId, patch: UpdateInvoice) -> Result<()>;
    async fn delete_invoice(&self, id: InvoiceId) -> Result<bool>;
}
