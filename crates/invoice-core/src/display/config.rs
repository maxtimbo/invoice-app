use std::fmt;

use crate::models::config::Config;

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SMTP Server:\t\t{}\n", self.smtp_server)?;
        write!(f, "Port:\t\t\t{}\n", self.port)?;
        write!(f, "TLS:\t\t\t{}\n", self.tls)?;
        write!(f, "Username:\t\t{}\n", self.username)?;
        write!(f, "Password:\t\t{}\n", self.password)?;
        write!(f, "From name:\t\t{}\n", self.fromname)
    }
}
