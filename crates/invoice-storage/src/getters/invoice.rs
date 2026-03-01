use crate::db::InvoiceDB;

use invoice_core::models::invoice::Invoice;
use invoice_core::models::attributes::InvoiceAttrs;
use invoice_core::models::stage::InvoiceStage;
use invoice_core::models::status::PaidStatus;

impl InvoiceDB {
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
            let items: HashMap<Item, i64> = items_vec
                .into_iter()
                .map(|item| {
                    let item_data = self.get_item(&item.item)?;
                    Ok((item_data, item.quantity))
                })
                .collect::<Result<HashMap<Item, i64>, rusqlite::Error>>()?;
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
}
