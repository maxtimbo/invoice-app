use crate::db::InvoiceDB;

use invoice_core::models::template::Template;

impl InvoiceDB {
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
}
