use std::fmt;

use invoice_core::models::client::Client;

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID:\t\t{}\n", self.id)?;
        write!(f, "Name:\t\t{}\n", self.name)?;
        write!(f, "Contact Information:\n{}", self.contact)
    }
}
