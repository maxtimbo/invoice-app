use anyhow::{anyhow, Result};
use async_trait::async_trait;
use sqlx::Row;

use crate::sqlite::SqliteStorage;
use invoice_app::ports::repos::terms_repo::{TermsRepo, CreateTerms, UpdateTerms};
use invoice_core::models::{
    terms::Terms,
    ids::TermsId,
};

#[async_trait]
impl TermsRepo for SqliteStorage {
    async fn get_terms(&self, id: TermsId) -> Result<Option<Terms>> {
        let row = sqlx::query("SELECT id, name, due FROM terms WHERE id = ?")
            .bind(id.0)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|r| Terms {
            id: TermsId(r.get::<i64, _>("id")),
            name: r.get::<String, _>("name"),
            due: r.get::<i64, _>("due"),
        }))
    }
    async fn list_terms(&self) -> Result<Vec<Terms>> {
        let rows = sqlx::query("SELECT id, name, due FROM terms ORDER BY id")
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter()
            .map(|r| Terms {
                id: TermsId(r.get::<i64, _>("id")),
                name: r.get::<String, _>("name"),
                due: r.get::<i64, _>("due"),
            })
            .collect())
    }
    async fn create_terms(&self, input: CreateTerms) -> Result<TermsId> {
        let res = sqlx::query("INSERT INTO terms (name, due) VALUES (?, ?)")
            .bind(input.name)
            .bind(input.due)
            .execute(&self.pool)
            .await?;
        Ok(TermsId(res.last_insert_rowid()))
    }
    async fn update_terms(&self, id: TermsId, patch: UpdateTerms) -> Result<()> {
        let mut terms = self.get_terms(id).await?.ok_or_else(|| anyhow!("terms {} not found", id.0))?;
        if let Some(name) = patch.name {
            terms.name = name;
        }
        if let Some(due) = patch.due {
            terms.due = due;
        }

        sqlx::query("UPDATE terms SET name = ?, due = ? WHERE id = ?")
            .bind(terms.name)
            .bind(terms.due)
            .bind(id.0)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    async fn delete_terms(&self, id: TermsId) -> Result<bool> {
        let res = sqlx::query("DELETE FROM terms WHERE id = ?")
            .bind(id.0)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected() > 0)
    }
}


