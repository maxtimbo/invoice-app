
use crate::db::InvoiceDB;

use invoice_core::models::config::Config;

impl InvoiceDB {
    pub fn get_config(&self) -> Result<Config, rusqlite::Error> {
        let query = "SELECT * FROM email_config WHERE id = ?";
        let config = self.connection.query_row(query, &[&0], |row| {
            Ok(Config {
                id: row.get(0)?,
                smtp_server: row.get(1)?,
                port: row.get(2)?,
                tls: row.get(3)?,
                username: row.get(4)?,
                password: row.get(5)?,
                fromname: row.get(6)?,
            })
        })?;
        Ok(config)
    }
}
