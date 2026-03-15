use std::collections::HashMap;

use chrono::{NaiveDate, Duration};
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};

use crate::models::ids::InvoiceId;
use crate::models::template::Template;
use crate::models::item::{Item, ItemDetail};
use crate::models::quantity::Quantity;
use crate::models::currency::Currency;
use crate::models::attributes::InvoiceAttrs;
use crate::models::status::PaidStatus;
use crate::models::stage::InvoiceStage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: InvoiceId,
    pub template: Template,
    pub attributes: InvoiceAttrs,
    pub date: NaiveDate,
    pub notes: Option<String>,
    pub items: HashMap<Item, Quantity>,
    pub message_sent: Option<NaiveDate>,
}

impl Invoice {
    pub fn calculate_subtotals(&self) -> Vec<ItemDetail> {
        let mut item_details: Vec<ItemDetail> = self.items
            .iter()
            .map(|(item, quantity)| {
                let subtotal_dec = item.rate.inner() * quantity.inner();
                ItemDetail {
                    id: item.id,
                    name: item.name.clone(),
                    rate: item.rate.clone(),
                    quantity: quantity.clone(),
                    subtotal: Currency::new(subtotal_dec),
                }
            })
            .collect();
        item_details.sort_by(|a, b| a.name.cmp(&b.name));
        item_details
    }
    pub fn calculate_total(&self) -> Currency {
        let total: Decimal = self
            .calculate_subtotals()
            .iter()
            .map(|d| d.subtotal.inner())
            .sum();
        Currency::new(total)
    }
    pub fn issue_date(&self) -> NaiveDate {
        self.date
    }
    pub fn due_date(&self) -> NaiveDate {
        self.issue_date() + Duration::days(self.template.terms.due)
    }
    pub fn email_subject(&self) -> String {
        let date = self.date.format("%B %d, %Y");
        match self.attributes.stage {
            InvoiceStage::Quote => format!("Quote #{:04} - {date}", self.id.0),
            InvoiceStage::Invoice => {
                let prefix = match &self.attributes.status {
                    PaidStatus::Waiting => String::new(),
                    PaidStatus::PastDue => "Past Due: ".to_string(),
                    PaidStatus::Paid { .. } => "Paid: ".to_string(),
                    PaidStatus::Failed { .. } => "Failed: ".to_string(),
                    PaidStatus::Refunded { .. } => "Refunded: ".to_string(),
                };
                format!("{prefix}Invoice #{:04} - {date}", self.id.0)
            }
        }
    }
}
