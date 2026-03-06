use chrono::NaiveDate;

use invoice_core::models::{
    ids::{InvoiceId, TemplateId, ItemId},
    attributes::InvoiceAttrs,
    quantity::Quantity,
};

pub(crate) struct InvoiceItemSkel {
    pub item_id: ItemId,
    pub quantity: Quantity,
}

pub(crate) struct InvoiceSkel {
    pub id: InvoiceId,
    pub template_id: TemplateId,
    pub date: NaiveDate,
    pub attributes: InvoiceAttrs,
    pub notes: Option<String>,
    pub items: Vec<InvoiceItemSkel>,
}
