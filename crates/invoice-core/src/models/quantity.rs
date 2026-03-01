use rust_decimal::Decimal;

use crate::errors::DomainError;

#[derive(Debug, Clone, PartialEq)]
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
}
