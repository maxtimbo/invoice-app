use std::fmt;

use crate::models::item::Item;

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (Rate: {})", self.name, self.rate)
    }
}
