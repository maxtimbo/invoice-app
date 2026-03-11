use std::fmt;

use crate::models::summary::InvoiceSummary;

impl fmt::Display for InvoiceSummary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID: {}\n", self.id)?;
        write!(f, "Client: {}\n", self.client_name)?;
        write!(f, "Date Issued: {}\n", self.issued)?;
        write!(f, "Due Date: {}\n", self.due)?;
        write!(f, "Payment Status: {}\n", self.status)?;
        write!(f, "Invoice Total: {}\n", self.total)?;
        Ok(())
    }
}
