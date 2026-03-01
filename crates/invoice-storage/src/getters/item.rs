use crate::db::InvoiceDB;

use invoice_core::models::item::Item;

impl InvoiceDB {
    pub fn get_item(&self, id: &i64) -> Result<Item, rusqlite::Error> {
        let query = "SELECT * FROM items WHERE id = ?";
        let item = self.connection.query_row(query, &[id], |row| {
            let rate: i64 = row.get(2)?;
            Ok(Item {
                id: row.get(0)?,
                name: row.get(1)?,
                rate: i64_to_decimal!(rate),
            })
        })?;
        Ok(item)
    }
}
