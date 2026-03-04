use anyhow::{anyhow, Result};
use async_trait::async_trait;
use sqlx::Row;
use serde_json;

use crate::sqlite::SqliteStorage;
use invoice_app::ports::repos::template_repo::{
    CreateTemplate,
    UpdateTemplate,
    TemplateRepo,
};
use invoice_core::models::{
    template::Template,
    ids::{
        TemplateId,
        CompanyId,
        ClientId,
        TermsId,
        MethodId,
    },
};

#[async_trait]
impl TemplateRepo for SqliteStorage {
    async fn get(&self, id: TemplateId) -> Result<Option<TemplateSkel>> {
        let row = sqlx::query(
            "SELECT
                id,
                name,
                company_id,
                client_id,
                terms_id,
                methods_json 
            FROM templates WHERE id = ?")
            .bind(id.0)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|r| {
            let methods_json: String = 
                r.get::<String, _>("methods_json");

            let methods_ids: Vec<i64> =
                serde_json::from_str(&methods_json)
                    .expect("invalid methods_json");
            let methods: Vec<MethodId> =
                method_ids.into_iter().map(MethodId).collect();

            TemplateSkel {
                id: TemplateId(r.get::<i64, _>("id")),
                name: r.get::<String, _>("name"),
                company_id: CompanyId(r.get::<i64, _>("company_id")),
                client_id: ClientId(r.get::<i64, _>("client_id")),
                terms_id: TermsId(r.get::<i64, _>("terms_id")),
                methods,
            }
        }))
    }
    async fn list(&self) -> Result<Vec<TemplateSkel>> {
        let rows = sqlx::query(
            "SELECT id FROM templates ORDER BY id")
            .fetch_all(&self.pool)
            .await?;
}

