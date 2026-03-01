
use anyhow::Result;
use std::collections::HashMap;

use crate::db::InvoiceDB;

use invoice_core::i64_to_decimal;
use invoice_core::models::shortlist::ShortList;

impl InvoiceDB {
    pub fn get_table(&self, table_name: &str) -> Result<Vec<ShortList>, rusqlite::Error> {
        let query = match table_name {
            "invoices" => format!("SELECT id, date FROM {}", table_name),
            _ => format!("SELECT id, name FROM {}", table_name),
        };

        let mut stmt = self.connection.prepare(&query)?;
        let short_list_iter = stmt.query_map([], |row| {
            Ok(ShortList {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        let results = short_list_iter.collect::<Result<Vec<ShortList>, rusqlite::Error>>()?;
        Ok(results)
    }
}
