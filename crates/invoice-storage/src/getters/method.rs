use crate::db::InvoiceDB;

use invoice_core::models::methods::Methods;

impl InvoiceDB {
    pub fn get_method(&self, id: &i64) -> Result<Methods, rusqlite::Error> {
        let query = "SELECT * FROM methods WHERE id = ?";
        let method = self.connection.query_row(query, &[id], |row| {
            Ok(Methods {
                id: row.get(0)?,
                name: row.get(1)?,
                link: row.get(2)?,
                qr: row.get(3)?,
            })
        })?;
        Ok(method)
    }
}
