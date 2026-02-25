use anyhow::Result;
use std::collections::HashMap;

use crate::db::InvoiceDB;

use invoice_core::i64_to_decimal;
use invoice_core::models::client::Client;
use invoice_core::models::company::Company;
use invoice_core::models::contact::Contact;
use invoice_core::models::config::Config;
use invoice_core::models::invoice::{Invoice, InvoiceItem, InvoiceAttrs, InvoiceStage, PaidStatus};
use invoice_core::models::template::Template;
use invoice_core::models::items::Items;
use invoice_core::models::methods::Methods;
use invoice_core::models::terms::Terms;
use invoice_core::models::ShortList;

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
    pub fn get_item(&self, id: &i64) -> Result<Items, rusqlite::Error> {
        let query = "SELECT * FROM items WHERE id = ?";
        let item = self.connection.query_row(query, &[id], |row| {
            let rate: i64 = row.get(2)?;
            Ok(Items {
                id: row.get(0)?,
                name: row.get(1)?,
                rate: i64_to_decimal!(rate),
            })
        })?;
        Ok(item)
    }
    pub fn get_template(&self, id: &i64) -> Result<Template, rusqlite::Error> {
        let query = "SELECT * FROM templates WHERE id = ?";
        let template = self.connection.query_row(query, &[id], |row| {
            let company_id: i64 = row.get(2)?;
            let client_id: i64 = row.get(3)?;
            let terms_id: i64 = row.get(4)?;
            let methods_json: String = row.get(5)?;

            let method_list: Vec<i64> =
                serde_json::from_str(&methods_json).expect("Failed to deserialize methods");

            let mut methods: Vec<Methods> = Vec::new();

            for method in method_list {
                let obj = self.get_method(&method)?;
                methods.push(obj);
            }

            Ok(Template {
                id: row.get(0)?,
                name: row.get(1)?,
                company: self.get_company(&company_id)?,
                client: self.get_client(&client_id)?,
                terms: self.get_terms(&terms_id)?,
                methods: methods,
            })
        })?;
        Ok(template)
    }
    pub fn get_invoice(&self, id: &i64) -> Result<Invoice, rusqlite::Error> {
        let query = "SELECT * FROM invoices WHERE id = ?";
        let invoice = self.connection.query_row(query, &[id], |row| {
            let template_id: i64 = row.get(1)?;
            let date: String = row.get(2)?;
            let show_methods: bool = match row.get(3)? {
                0 => false,
                1 => true,
                _ => return Err(rusqlite::Error::InvalidQuery)
            };
            let show_notes: bool = match row.get(4)? {
                0 => false,
                1 => true,
                _ => return Err(rusqlite::Error::InvalidQuery)
            };
            let stage = match row.get::<_, String>(5)?.as_str() {
                "Invoice" => InvoiceStage::Invoice,
                "Quote" => InvoiceStage::Quote,
                _ => {
                    println!("error parsing invoice");
                    return Err(rusqlite::Error::InvalidQuery);
                }
            };
            let status_str: String = row.get(6)?;
            let status_date: Option<String> = row.get(7)?;
            let status_check: Option<String> = row.get(8)?;

            let status = match status_str.as_str() {
                "Waiting" => PaidStatus::Waiting,
                "Past Due" => PaidStatus::PastDue,
                "Paid" => PaidStatus::Paid {
                    date: status_date.unwrap_or_else(|| "Unknown".to_string()),
                    check: status_check
                },
                "Failed" => PaidStatus::Failed {
                    date: status_date.unwrap_or_else(|| "Unknown".to_string())
                },
                "Refunded" => PaidStatus::Refunded {
                    date: status_date.unwrap_or_else(|| "Unknown".to_string()),
                },
                _ => return Err(rusqlite::Error::InvalidQuery)
            };

            let notes: Option<String> = row.get(9)?;
            let items_str: String = row.get(10)?;

            let attributes = InvoiceAttrs{
                show_methods,
                show_notes,
                stage,
                status
            };

            let items_vec: Vec<InvoiceItem> = serde_json::from_str(&items_str)
                .map_err(|_| rusqlite::Error::ExecuteReturnedResults)?;
            let items: HashMap<Items, i64> = items_vec
                .into_iter()
                .map(|item| {
                    let item_data = self.get_item(&item.item)?;
                    Ok((item_data, item.quantity))
                })
                .collect::<Result<HashMap<Items, i64>, rusqlite::Error>>()?;
            Ok(Invoice {
                id: row.get(0)?,
                template: self.get_template(&template_id)?,
                date,
                attributes,
                notes,
                items,
            })
        })?;
        Ok(invoice)
    }

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
