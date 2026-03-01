use crate::db::InvoiceDB;

use invoice_core::models::company::Company;
use invoice_core::models::contact::Contact;

impl InvoiceDB {
    pub fn get_company(&self, id: &i64) -> Result<Company, rusqlite::Error> {
        let query = "SELECT * FROM company WHERE id = ?";
        let company = self.connection.query_row(query, &[id], |row| {
            Ok(Company {
                id: row.get(0)?,
                name: row.get(1)?,
                logo: row.get(2)?,
                contact: Contact {
                    phone: row.get(3)?,
                    email: row.get(4)?,
                    addr1: row.get(5)?,
                    addr2: row.get(6)?,
                    city: row.get(7)?,
                    state: row.get(8)?,
                    zip: row.get(9)?,
                },
            })
        })?;
        Ok(company)
    }
}
