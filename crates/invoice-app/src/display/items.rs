use std::fmt;

use invoice_core::models::items::Items;

impl fmt::Display for Items {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (Rate: {})", self.name, self.rate)
    }
}
