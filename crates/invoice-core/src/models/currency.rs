use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Currency(pub Decimal);

impl Currency {
    pub fn new(amount: Decimal) -> Self {
        Self(amount.round_dp(2))
    }

    pub fn zero() -> Self {
        Self(Decimal::ZERO)
    }

    pub fn add(&self, other: &Currency) -> Currency {
        Currency::new(self.0 + other.0)
    }

    pub fn inner(&self) -> Decimal {
        self.0
    }
    pub fn to_cents(&self) -> i64 {
        (self.0 * Decimal::from(100))
            .round()
            .to_i64()
            .expect("Value too large or NaN")
    }
    pub fn from_cents(cents: i64) -> Self {
        Currency::new(Decimal::from(cents) / Decimal::from(100))
    }
}
