use std::fmt;

use crate::models::quantity::Quantity;

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0) 
    }
}
