mod views;
mod actions;

pub use views::*;
pub use actions::*;

use serde::{Serialize, Deserialize};
use invoice_core::models::contact::Contact;

#[derive(Serialize, Deserialize)]
pub struct ClientForm {
    pub name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub addr1: Option<String>,
    pub addr2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
}

impl ClientForm {
    pub fn into_contact(self) -> (String, Contact) {
        (self.name, Contact {
            phone: self.phone,
            email: self.email,
            addr1: self.addr1,
            addr2: self.addr2,
            city: self.city,
            state: self.state,
            zip: self.zip,
        })
    }
}
