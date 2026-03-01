use std::fmt;

use crate::models::summary::InvoiceSummary;

impl fmt::Display for InvoiceSummary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID: {}", self.id)?;
        write!(f, "Client: {}", self.client_name)?;
        write!(f, "Date Issued: {}", self.issued)?;
        write!(f, "Due Date: {}", self.due)?;
        write!(f, "Payment Status: {}", self.status)?;
        write!(f, "Invoice Total: {}", self.total)?;
        Ok(())
    }
}
