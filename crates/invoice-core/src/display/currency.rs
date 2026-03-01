use std::fmt;

use crate::models::currency::Currency;


impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0) 
    }
}
