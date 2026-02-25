use std::fmt;

use invoice_core::models::shortlist::ShortList;

impl fmt::Display for ShortList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID: {}, Name: {}", self.id, self.name)
    }
}
