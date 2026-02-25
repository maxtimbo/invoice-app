use crate::db::cached::CachedStmt;
use crate::db::InvoiceDB;

use anyhow::Result;

impl InvoiceDB {
    pub fn create_entry(&self, cache: CachedStmt) -> Result<i64> {
        let mut stmt = self.connection.prepare(&cache.query)?;
        stmt.execute(rusqlite::params_from_iter(&cache.params))?;
        let new_id = self.connection.last_insert_rowid();
        Ok(new_id)
    }
    pub fn update_entry(&self, cache: CachedStmt, id: &i64) -> Result<()> {
        let mut stmt = self.connection.prepare(&cache.query)?;
        stmt.execute(rusqlite::params_from_iter(&cache.params))?;
        self.print_entry(&cache.table.as_str(), &id)?;
        Ok(())
    }
    pub fn delete_entry(&self, cache: CachedStmt, id: &i64) -> Result<()> {
        let mut stmt = self.connection.prepare(&cache.query)?;
        stmt.execute(&[id])?;
        Ok(())
    }
    pub fn print_entry(&self, table: &str, new_id: &i64) -> Result<()> {
        match table {
            "email_config" => {
                let new_entry = self.get_config()?;
                println!("{}", new_entry);
            }
            "company" => {
                let new_entry = self.get_company(&new_id)?;
                println!("{}", new_entry);
            }
            "client" => {
                let new_entry = self.get_client(&new_id)?;
                println!("{}", new_entry);
            }
            "terms" => {
                let new_entry = self.get_terms(&new_id)?;
                println!("{}", new_entry);
            }
            "methods" => {
                let new_entry = self.get_method(&new_id)?;
                println!("{}", new_entry);
            }
            "items" => {
                let new_entry = self.get_item(&new_id)?;
                println!("{}", new_entry);
            }
            "templates" => {
                let new_entry = self.get_template(&new_id)?;
                println!("{}", new_entry);
            }
            "invoices" => {
                let new_entry = self.get_invoice(&new_id)?;
                println!("{}", new_entry);
            }
            _ => todo!("Something else!"),
        }
        Ok(())
    }
}
