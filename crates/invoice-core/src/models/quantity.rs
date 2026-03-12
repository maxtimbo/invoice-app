use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use serde::{Serialize, Deserialize};

use crate::errors::DomainError;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quantity(pub Decimal);

impl Quantity {
    pub fn new(value: Decimal) -> Result<Self, DomainError> {
        if value <= Decimal::ZERO {
            return Err(DomainError::InvalidQuantity);
        }
        Ok(Self(value))
    }

    pub fn inner(&self) -> Decimal {
        self.0
    }

    pub fn to_scaled(&self) -> i64 {
        (self.0 * Decimal::from(1000))
            .round()
            .to_i64()
            .expect("Value too large or NaN")
    }
    pub fn from_scaled(n: i64) -> Result<Self, DomainError> {
        Quantity::new(Decimal::from(n) / Decimal::from(1000))
    }
}
