use std::collections::HashMap;

use chrono::{NaiveDate, Duration};
use rust_decimal::Decimal;

use crate::models::ids::InvoiceId;
use crate::models::template::Template;
use crate::models::item::{Item, ItemDetail};
use crate::models::quantity::Quantity;
use crate::models::currency::Currency;
use crate::models::attributes::InvoiceAttrs;

#[derive(Debug, Clone)]
pub struct Invoice {
    pub id: InvoiceId,
    pub template: Template,
    pub attributes: InvoiceAttrs,
    pub date: NaiveDate,
    pub notes: Option<String>,
    pub items: HashMap<Item, Quantity>,
}

impl Invoice {
    pub fn calculate_subtotals(&self) -> Vec<ItemDetail> {
        let mut item_details: Vec<ItemDetail> = self.items
            .iter()
            .map(|(item, quantity)| {
                let subtotal_dec = item.rate.inner() * quantity.inner();
                ItemDetail {
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
}
