use anyhow::{anyhow, Result};
use async_trait::async_trait;
use sqlx::Row;

use crate::sqlite::SqliteStorage;
use invoice_app::ports::repos::method_repo::{
    MethodRepo,
    CreateMethod,
    UpdateMethod
};
use invoice_core::models::{
    method::Method,
    ids::MethodId,
};

#[async_trait]
impl MethodRepo for SqliteStorage {
    async fn get_method(&self, id: MethodId) -> Result<Option<Method>> {
        let row = sqlx::query("SELECT id, name, link, qr FROM methods WHERE id = ?")
            .bind(id.0)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|r| Method {
            id: MethodId(r.get::<i64, _>("id")),
            name: r.get::<String, _>("name"),
            link: Some(r.get::<String, _>("link")),
            qr: Some(r.get::<Vec<u8>, _>("qr")),
        }))
    }

    async fn list_method(&self) -> Result<Vec<Method>> {
        let rows = sqlx::query("SELECT id, name, link, qr FROM methods ORDER BY id")
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter()
            .map(|r| Method {
                id: MethodId(r.get::<i64, _>("id")),
                name: r.get::<String, _>("name"),
                link: Some(r.get::<String, _>("link")),
                qr: Some(r.get::<Vec<u8>, _>("qr")),
            })
            .collect())
    }
    async fn create_method(&self, input: CreateMethod) -> Result<MethodId> {
        let res = sqlx::query("INSERT INTO methods (name, link, qr) VALUES (?, ?, ?)")
            .bind(input.name)
            .bind(input.link)
            .bind(input.qr)
            .execute(&self.pool)
            .await?;
        Ok(MethodId(res.last_insert_rowid()))
    }
    async fn update_method(&self, id: MethodId, patch: UpdateMethod) -> Result<()> {
        let mut method = self.get_method(id).await?.ok_or_else(|| anyhow!("Method {} not found", id.0))?;
        if let Some(name) = patch.name {
            method.name = name;
        }
        if let Some(link) = patch.link {
            method.link = Some(link);
        }
        if let Some(qr) = patch.qr {
            method.qr = Some(qr);
        }

        sqlx::query("UPDATE methods SET name = ?, link = ?, qr = ? WHERE id = ?")
            .bind(method.name)
            .bind(method.link)
            .bind(method.qr)
            .bind(id.0)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    async fn delete_method(&self, id: MethodId) -> Result<bool> {
        let res = sqlx::query("DELETE FROM methods WHERE id = ?")
            .bind(id.0)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected() > 0)
    }
}
