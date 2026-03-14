mod views;
mod actions;

pub use views::*;
pub use actions::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct InvoiceSummaryView {
    pub id: i64,
    pub client_name: String,
    pub issued: String,
    pub due: String,
    pub status: String,
    pub total: String,
}

#[derive(Serialize)]
pub struct InvoiceItemView {
    pub id: i64,
    pub name: String,
    pub rate: String,
    pub quantity: String,
    pub subtotal: String,
}

#[derive(Serialize)]
pub struct InvoiceEditView {
    pub id: i64,
    pub template_id: i64,
    pub date: String,
    pub client_name: String,
    pub company_name: String,
    pub show_methods: bool,
    pub show_notes: bool,
    pub stage: String,
    pub status: String,
    pub status_date: Option<String>,
    pub status_check: Option<String>,
    pub notes: Option<String>,
    pub items: Vec<InvoiceItemView>,
    pub total: String,
}

#[derive(Deserialize)]
pub struct UpdateInvoiceForm {
    pub show_methods: Option<String>,
    pub show_notes: Option<String>,
    pub stage: String,
    pub status: String,
    pub status_date: Option<String>,
    pub status_check: Option<String>,
    pub notes: Option<String>,
}
