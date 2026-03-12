
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvoiceStage {
    Quote,
    Invoice,
}
