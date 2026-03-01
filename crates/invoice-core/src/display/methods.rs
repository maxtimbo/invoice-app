use std::fmt;

use crate::models::methods::Methods;

impl fmt::Display for Methods {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID:\t\t{}\n", self.id)?;
        write!(f, "Name:\t\t{}\n", self.name)?;
        if let Some(ref link) = self.link {
            write!(f, "Link:\t\t{}\n", link)?;
        } else {
            write!(f, "Link:\t\tNone\n")?;
        }
        write!(f, "Has QR:\t\t{}", self.qr.is_some())
    }
}
