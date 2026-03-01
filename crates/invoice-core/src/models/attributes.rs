use crate::models::status::PaidStatus;
use crate::models::stage::InvoiceStage;

#[derive(Debug, Clone)]
pub struct InvoiceAttrs {
    pub show_methods: bool,
    pub show_notes: bool,
    pub stage: InvoiceStage,
    pub status: PaidStatus,
}
