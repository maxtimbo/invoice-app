use crate::db::InvoiceDB;

use invoice_core::models::terms::Terms;

impl InvoiceDB {
    pub fn get_terms(&self, id: &i64) -> Result<Terms, rusqlite::Error> {
        let query = "SELECT * FROM terms WHERE id = ?";
        let terms = self.connection.query_row(query, &[id], |row| {
            Ok(Terms {
                id: row.get(0)?,
                name: row.get(1)?,
                due: row.get(2)?,
            })
        })?;
        Ok(terms)
    }
}
