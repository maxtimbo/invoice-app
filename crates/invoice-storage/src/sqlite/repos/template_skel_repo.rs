use anyhow::{anyhow, Result};
use async_trait::async_trait;
use sqlx::Row;
use serde_json;

use crate::sqlite::SqliteStorage;
use invoice_app::ports::repos::template_skel_repo::{
    CreateTemplateSkel,
    UpdateTemplateSkel,
    TemplateSkelRepo,
};
use invoice_core::models::{
    template_skel::TemplateSkel,
    ids::{
        TemplateId,
        CompanyId,
        ClientId,
        TermsId,
        MethodId,
    }
};

#[async_trait]
impl TemplateSkelRepo for SqliteStorage {
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

            let method_vec: Vec<i64> =
                serde_json::from_str(&methods_json)
                    .expect("invalid methods_json");
            let method_ids: Vec<MethodId> =
                method_vec.into_iter().map(MethodId).collect();

            TemplateSkel {
                id: TemplateId(r.get::<i64, _>("id")),
                name: r.get::<String, _>("name"),
                company_id: CompanyId(r.get::<i64, _>("company_id")),
                client_id: ClientId(r.get::<i64, _>("client_id")),
                terms_id: TermsId(r.get::<i64, _>("terms_id")),
                method_ids,
            }
        }))
    }
    async fn list(&self) -> Result<Vec<TemplateSkel>> {
        let rows = sqlx::query(
            "SELECT
                id,
                name,
                company_id,
                client_id,
                terms_id,
                methods_json
            FROM templates ORDER BY id")
            .fetch_all(&self.pool)
            .await?;

        let templates = rows
            .into_iter()
            .map(|r| {
                let methods_json: String = r.get::<String, _>("methods_json");
                let method_vec: Vec<i64> =
                    serde_json::from_str(&methods_json)
                        .expect("invalid methods_json");
                let method_ids: Vec<MethodId> =
                    method_vec.into_iter().map(MethodId).collect();

                TemplateSkel {
                    id: TemplateId(r.get::<i64, _>("id")),
                    name: r.get::<String, _>("name"),
                    company_id: CompanyId(r.get::<i64, _>("company_id")),
                    client_id: ClientId(r.get::<i64, _>("client_id")),
                    terms_id: TermsId(r.get::<i64, _>("terms_id")),
                    method_ids,
                }
            })
            .collect();
        Ok(templates)
    }
    async fn create(&self, input: CreateTemplateSkel) -> Result<TemplateId> {
        let methods_json = serde_json::to_string(
            &input.methods.iter().map(|m| m.0).collect::<Vec<i64>>()
        )?;

        let id = sqlx::query(
            "INSERT INTO templates
                (
                    name,
                    company_id,
                    client_id,
                    terms_id,
                    methods_json
                )
                VALUES (?, ?, ?, ?, ?)")
            .bind(&input.name)
            .bind(input.company.0)
            .bind(input.client.0)
            .bind(input.terms.0)
            .bind(&methods_json)
            .execute(&self.pool)
            .await?
            .last_insert_rowid();
        Ok(TemplateId(id))
    }
    async fn update(&self, id: TemplateId, patch: UpdateTemplateSkel) -> Result<()> {
        let mut template = self.get(id).await?.ok_or_else(|| anyhow!("template {} not found", id.0))?;
        if let Some(name) = patch.name {
            template.name = name;
        }
        if let Some(company) = patch.company {
            template.company_id = company;
        }
        if let Some(client) = patch.client {
            template.client_id = client;
        }
        if let Some(terms) = patch.terms {
            template.terms_id = terms;
        }
        if let Some(methods) = patch.methods {
            template.method_ids = methods;
        }
        let methods_json = serde_json::to_string(
                &template.method_ids.iter().map(|m| m.0).collect::<Vec<i64>>()
            )?;

        sqlx::query(
            "UPDATE templates
            SET 
                name = ?,
                company_id = ?,
                client_id = ?,
                terms_id = ?,
                methods_json = ?
            WHERE id = ?")
            .bind(template.name)
            .bind(template.company_id.0)
            .bind(template.client_id.0)
            .bind(template.terms_id.0)
            .bind(&methods_json)
            .bind(id.0)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
    async fn delete(&self, id: TemplateId) -> Result<bool> {
        let res = sqlx::query("DELETE FROM templates WHERE id = ?")
            .bind(id.0)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected() > 0)
    }

}
