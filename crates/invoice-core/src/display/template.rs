use std::fmt;

use crate::models::template::Template;

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID:\t\t{}\n", self.id)?;
        write!(f, "Name:\t\t{}\n\n", self.name)?;
        write!(f, "Company Information:\n{}\n", self.company)?;
        write!(f, "Client Information:\n{}\n", self.client)?;
        write!(f, "Terms:\n")?;
        write!(f, "{}\n", self.terms)?;
        write!(f, "Payment Methods:\n")?;
        for method in &self.methods {
            write!(f, "{}\n", method)?;
        }
        Ok(())
    }
}
