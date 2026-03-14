mod views;
mod actions;

pub use views::*;
pub use actions::*;

use serde::Deserialize;
use rust_decimal::Decimal;
use std::str::FromStr;
use invoice_core::models::currency::Currency;

#[derive(Deserialize)]
pub struct ItemForm {
    pub name: String,
    pub rate: String,
}

impl ItemForm {
    pub fn parse_rate(&self) -> Result<Currency, String> {
        Decimal::from_str(&self.rate)
            .map(Currency::new)
            .map_err(|_| format!("Invalid rate: {}", self.rate))
    }
}
