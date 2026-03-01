use crate::db::InvoiceDB;

use invoice_core::models::client::Client;
use invoice_core::models::contact::Contact;

impl InvoiceDB {
    pub fn get_client(&self, id: &i64) -> Result<Client, rusqlite::Error> {
        let query = "SELECT * FROM client WHERE id = ?";
        let client = self.connection.query_row(query, &[id], |row| {
            Ok(Client {
                id: row.get(0)?,
                name: row.get(1)?,
                contact: Contact {
                    phone: row.get(2)?,
                    email: row.get(3)?,
                    addr1: row.get(4)?,
                    addr2: row.get(5)?,
                    city: row.get(6)?,
                    state: row.get(7)?,
                    zip: row.get(8)?,
                },
            })
        })?;
        Ok(client)
    }
}
