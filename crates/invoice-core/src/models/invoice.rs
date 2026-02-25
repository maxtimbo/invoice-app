use std::collections::HashMap;

use crate::models::template::Template;
use crate::models::items::Items;

pub struct Invoice {
    pub id: i64,
    pub template: Template,
    pub attributes: InvoiceAttrs,
    pub date: String,
    pub notes: Option<String>,
    pub items: HashMap<Items, i64>,
}

pub struct InvoiceAttrs {
    pub show_methods: bool,
    pub show_notes: bool,
    pub stage: InvoiceStage,
    pub status: PaidStatus,
}

pub enum InvoiceStage {
    Quote,
    Invoice,
}

pub enum PaidStatus {
    Waiting,
    PastDue,
    Paid { date: String, check: Option<String> },
    Failed { date: String},
    Refunded { date: String},
}
