
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use sqlx::Row;
use serde_json;

use crate::sqlite::SqliteStorage;
use crate::sqlite::models::template_skel::TemplateSkel;
use invoice_app::ports::repos::company_repo::CompanyRepo;
use invoice_app::ports::repos::client_repo::ClientRepo;
use invoice_app::ports::repos::terms_repo::TermsRepo;
use invoice_app::ports::repos::method_repo::MethodRepo;
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
    }
};

impl SqliteStorage {
    async fn fetch_skel(&self, id: TemplateId) -> Result<Option<TemplateSkel>> {
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
            let methods_json: String = r.get::<String, _>("methods_json");
            let method_vec: Vec<i64> = serde_json::from_str(&methods_json).expect("invalid methods_json");
            let method_ids: Vec<MethodId> = method_vec.into_iter().map(MethodId).collect();

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
    async fn hydrate_template(&self, skel: TemplateSkel) -> Result<Template> {
        let company = self.get_company(skel.company_id).await?
            .ok_or_else(|| anyhow!("company {} not found", skel.company_id.0))?;
        let client = self.get_client(skel.client_id).await?
            .ok_or_else(|| anyhow!("client {} not found", skel.client_id.0))?;
        let terms = self.get_terms(skel.terms_id).await?
            .ok_or_else(|| anyhow!("terms {} not found", skel.terms_id.0))?;

        let mut method = Vec::new();
        for method_id in skel.method_ids {
            let m = self.get_method(method_id).await?
                .ok_or_else(|| anyhow!("method {} not found", method_id.0))?;
            method.push(m);
        }
        Ok(Template {
            id: skel.id,
            name: skel.name,
            company,
            client,
            terms,
            method,
        })
    }

}

#[async_trait]
impl TemplateRepo for SqliteStorage {
    async fn get_template(&self, id: TemplateId) -> Result<Option<Template>> {
        match self.fetch_skel(id).await? {
            Some(skel) => Ok(Some(self.hydrate_template(skel).await?)),
            None => Ok(None),
        }
    }
    async fn list_template(&self) -> Result<Vec<Template>> {
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

        let mut templates = Vec::new();

        for r in rows {
            let methods_json: String = r.get::<String, _>("methods_json");
            let method_vec: Vec<i64> = serde_json::from_str(&methods_json).expect("invalid methods_json");
            let method_ids = method_vec.into_iter().map(MethodId).collect();

            let skel = TemplateSkel {
                id: TemplateId(r.get::<i64, _>("id")),
                name: r.get::<String, _>("name"),
                company_id: CompanyId(r.get::<i64, _>("company_id")),
                client_id: ClientId(r.get::<i64, _>("client_id")),
                terms_id: TermsId(r.get::<i64, _>("terms_id")),
                method_ids,
            };
            templates.push(self.hydrate_template(skel).await?);
        }
        Ok(templates)
    }
    async fn create_template(&self, input: CreateTemplate) -> Result<TemplateId> {
        let methods_json = serde_json::to_string(
            &input.method.iter().map(|m| m.0).collect::<Vec<i64>>()
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
    async fn update_template(&self, id: TemplateId, patch: UpdateTemplate) -> Result<()> {
        let mut skel = self.fetch_skel(id).await?
            .ok_or_else(|| anyhow!("template {} not found", id.0))?;
        if let Some(name) = patch.name {
            skel.name = name;
        }
        if let Some(company) = patch.company {
            skel.company_id = company;
        }
        if let Some(client) = patch.client {
            skel.client_id = client;
        }
        if let Some(terms) = patch.terms {
            skel.terms_id = terms;
        }
        if let Some(methods) = patch.method {
            skel.method_ids = methods;
        }
        let methods_json = serde_json::to_string(
                &skel.method_ids.iter().map(|m| m.0).collect::<Vec<i64>>()
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
            .bind(skel.name)
            .bind(skel.company_id.0)
            .bind(skel.client_id.0)
            .bind(skel.terms_id.0)
            .bind(&methods_json)
            .bind(id.0)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
    async fn delete_template(&self, id: TemplateId) -> Result<bool> {
        let res = sqlx::query("DELETE FROM templates WHERE id = ?")
            .bind(id.0)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected() > 0)
    }

}
