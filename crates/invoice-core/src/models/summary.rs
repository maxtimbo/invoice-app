use chrono::NaiveDate;

use crate::models::status::PaidStatus;
use crate::models::ids::InvoiceId;
use crate::models::currency::Currency;

#[derive(Debug, Clone)]
pub struct InvoiceSummary {
    pub id: InvoiceId,
    pub client_name: String,
    pub issued: NaiveDate,
    pub due: NaiveDate,
    pub status: PaidStatus,
    pub total: Currency,
}
