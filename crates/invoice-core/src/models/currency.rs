use rust_decimal::Decimal;

#[derive(Debug, Clone, PartialEq)]
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
}
