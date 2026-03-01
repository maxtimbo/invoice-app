pub mod models;
pub mod display;
mod errors;

#[macro_export]
macro_rules! decimal_to_i64 {
    ($decimal:expr) => {{
        use rust_decimal::Decimal;
        use rust_decimal::prelude::ToPrimitive;
        let factor = Decimal::from(100);
        let scaled_decimal = $decimal * factor;
        scaled_decimal.round().to_i64().expect("Value too large or NaN")
    }};
}

#[macro_export]
macro_rules! i64_to_decimal {
    ($integer:expr) => {{
        use rust_decimal::Decimal;
        Decimal::from($integer) / Decimal::from(100)
    }};
}
