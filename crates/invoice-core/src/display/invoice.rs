use std::fmt;

use crate::models::invoice::Invoice;
use crate::models::stage::InvoiceStage;

impl fmt::Display for InvoiceStage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Quote => write!(f, "Quote"),
            Self::Invoice => write!(f, "Invoice"),
        }
    }
}

impl fmt::Display for Invoice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID:\t\t{}\n", self.id)?;
        write!(f, "Date:\t\t{}\n\n", self.date)?;
        write!(f, "Template Information:\n{}\n", self.template)?;
        match self.attributes.stage {
            InvoiceStage::Quote => {
                write!(f, "Stage:\t\tQuote\n")?;
            },
            InvoiceStage::Invoice => {
                write!(f, "Stage:\t\tInvoice\n")?;
            }
        }

        write!(f, "Payment status:\t{}", self.attributes.status)?;
        if let Some(notes) = &self.notes {
            write!(f, "Notes:\n{}\n\n", notes.to_string())?;
        }

        write!(f, "Invoice attributes:\n")?;
        write!(f, "Show notes:\t\t{}\n", self.attributes.show_notes)?;
        write!(f, "Show payment methods:\t{}\n\n", self.attributes.show_methods)?;

        write!(f, "Invoice Items:\n")?;
        write!(f, "Item\t\t\t\t| Rate\t| Quantity\t| Subtotal\n")?;
        for item in &self.calculate_subtotals() {
            write!(f, "{}\t| {}\t| ${}\t\t| ${}\n",
                        item.name,
                        item.quantity,
                        item.rate,
                        item.subtotal)?;
        }
        write!(f, "\t\t\t\t\tTotal:\t  ${}\n", &self.calculate_total())?;
        write!(f, "Due Date: {}", &self.due_date().format("%B %d, %Y").to_string())?;
        Ok(())
    }
}
