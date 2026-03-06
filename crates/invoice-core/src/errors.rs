use std::fmt;

#[derive(Debug)]
pub enum DomainError {
    InvalidQuantity,
    InvalidCurrency,
    InvalidStateTransition,
}


impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DomainError::InvalidQuantity => write!(f, "quantity must be greater than zero"),
            DomainError::InvalidCurrency => write!(f, "currency"),
            DomainError::InvalidStateTransition => write!(f, "state transition"),
        }
    }
}

impl std::error::Error for DomainError {}
