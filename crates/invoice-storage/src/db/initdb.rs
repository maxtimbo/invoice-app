use anyhow::{Context, Result};

use crate::db::InvoiceTx;

impl<'conn> InvoiceTx<'conn> {
    pub fn initdb(&self) -> Result<()> {
        self.tx.execute(
            "CREATE TABLE IF NOT EXISTS migrations (
                version INTEGER PRIMARY KEY
            );", [])
            .context("Failed to create migrations")?;
        self.tx
            .execute(
                "CREATE TABLE IF NOT EXISTS company (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                logo BLOB,
                phone TEXT,
                email TEXT,
                addr1 TEXT,
                addr2 TEXT,
                city TEXT,
                state TEXT,
                zip TEXT
            )",
                [],
            )
            .context("failed to create company table")?;
        self.tx
            .execute(
                "CREATE TABLE IF NOT EXISTS client (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                phone TEXT,
                email TEXT,
                addr1 TEXT,
                addr2 TEXT,
                city TEXT,
                state TEXT,
                zip TEXT
            )",
                [],
            )
            .context("failed to create client table")?;
        self.tx
            .execute(
                "CREATE TABLE IF NOT EXISTS items (
                 id INTEGER PRIMARY KEY AUTOINCREMENT,
                 name TEXT NOT NULL UNIQUE,
                 rate INTEGER
             )",
                [],
            )
            .context("failed to create item table")?;
        self.tx
            .execute(
                "CREATE TABLE IF NOT EXISTS terms (
                 id INTEGER PRIMARY KEY AUTOINCREMENT,
                 name TEXT NOT NULL UNIQUE,
                 due INTEGER NOT NULL UNIQUE
             )",
                [],
            )
            .context("failed to create terms table")?;
        self.tx
            .execute(
                "CREATE TABLE IF NOT EXISTS methods (
                 id INTEGER PRIMARY KEY AUTOINCREMENT,
                 name NOT NULL UNIQUE,
                 link TEXT,
                 qr BLOB
             )",
                [],
            )
            .context("failed to create methods table")?;
        self.tx
            .execute(
                "CREATE TABLE IF NOT EXISTS templates (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                company_id INTEGER NOT NULL,
                client_id INTEGER NOT NULL,
                terms_id INTEGER NOT NULL,
                methods_json TEXT NOT NULL,
                FOREIGN KEY (company_id)
                    REFERENCES company (id)
                    ON DELETE NO ACTION
                    ON UPDATE NO ACTION,
                FOREIGN KEY (client_id)
                    REFERENCES client (id)
                    ON DELETE NO ACTION
                    ON UPDATE NO ACTION,
                FOREIGN KEY (terms_id)
                    REFERENCES terms (id)
                    ON DELETE NO ACTION
                    ON UPDATE NO ACTION
            )",
                [],
            )
            .context("failed to create template table")?;
        self.tx
            .execute(
                "CREATE TABLE IF NOT EXISTS invoices (
                 id INTEGER PRIMARY KEY AUTOINCREMENT,
                 template_id INTEGER NOT NULL,
                 date TEXT NOT NULL,
                 show_methods INTEGER NOT NULL,
                 show_notes INTEGER NOT NULL,
                 stage TEXT NOT NULL,
                 status TEXT NOT NULL,
                 status_date TEXT,
                 status_check TEXT,
                 notes TEXT,
                 items_json TEXT NOT NULL,
                 FOREIGN KEY (template_id)
                     REFERENCES templates (id)
                     ON DELETE NO ACTION
                     ON UPDATE NO ACTION
             )",
                [],
            )
            .context("failed to create invoices table")?;
        self.tx.execute(
            "CREATE TABLE IF NOT EXISTS email_config (
                id INTEGER PRIMARY KEY CHECK (id = 0),
                smtp_server TEXT NOT NULL,
                port INTEGER NOT NULL,
                tls INTEGER NOT NULL,
                username TEXT NOT NULL,
                password TEXT NOT NULL,
                fromname TEXT NOT NULL
            );", [])
            .context("failed to create email_config table")?;

        Ok(())
    }
}
