use std::fmt;

impl fmt::Display for Contact {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref phone) = self.phone {
            write!(f, "Phone:\t\t{}\n", phone)?;
        }
        if let Some(ref email) = self.email {
            write!(f, "Email:\t\t{}\n", email)?;
        }
        if let Some(ref addr1) = self.addr1 {
            write!(f, "Addr1:\t\t{}\n", addr1)?;
        }
        if let Some(ref addr2) = self.addr2 {
            write!(f, "Addr2:\t\t{}\n", addr2)?;
        }
        if let Some(ref city) = self.city {
            write!(f, "City:\t\t{}\n", city)?;
        }
        if let Some(ref state) = self.state {
            write!(f, "State:\t\t{}\n", state)?;
        }
        if let Some(ref zip) = self.zip {
            write!(f, "Zip:\t\t{}\n", zip)?;
        }
        Ok(())
    }
}
