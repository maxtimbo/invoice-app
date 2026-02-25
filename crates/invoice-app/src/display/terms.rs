use std::fmt;

use invoice_core::models::terms::Terms;

impl fmt::Display for Terms {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: {} - Name: {}, Due: {}",
            self.id, self.name, self.due
        )
    }
}
