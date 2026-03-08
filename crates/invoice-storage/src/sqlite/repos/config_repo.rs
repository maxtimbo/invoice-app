use async_trait::async_trait;
use anyhow::Result;
use sqlx::Row;

use invoice_app::ports::repos::config_repo::{
    ConfigRepo,
    CreateConfig,
    UpdateConfig
};
use invoice_core::models::config::Config;

use crate::sqlite::SqliteStorage;

#[async_trait]
impl ConfigRepo for SqliteStorage {
    async fn get_config(&self) -> Result<Option<Config>> {
        let row = sqlx::query(
            "SELECT
                id,
                smtp_server,
                port,
                tls,
                username,
                password,
                fromname
            FROM email_config WHERE id = 1",
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Config {
            id:             r.get::<i64, _>("id"),
            smtp_server:    r.get("smtp_server"),
            port:           r.get::<i64, _>("port") as u16,
            tls:            r.get::<i64, _>("tls") != 0,
            username:       r.get("username"),
            password:       r.get("password"),
            fromname:       r.get("fromname"),
        }))
    }

    async fn create_config(&self, input: CreateConfig) -> Result<()> {
        sqlx::query(
            "INSERT INTO email_config (
                id,
                smtp_server,
                port,
                tls,
                username,
                password,
                fromname)
            VALUES (1, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&input.smtp_server)
        .bind(input.port as i64)
        .bind(input.tls as i64)
        .bind(&input.username)
        .bind(&input.password)
        .bind(&input.fromname)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_config(&self, patch: UpdateConfig) -> Result<()> {
        let mut sets: Vec<&str> = Vec::new();
        if patch.smtp_server.is_some()      { sets.push("smtp_server = ?"); }
        if patch.port.is_some()             { sets.push("port = ?"); }
        if patch.tls.is_some()              { sets.push("tls = ?"); }
        if patch.username.is_some()         { sets.push("username = ?"); }
        if patch.password.is_some()         { sets.push("password = ?"); }
        if patch.fromname.is_some()         { sets.push("fromname = ?"); }

        if sets.is_empty() {
            return Ok(());
        }

        let sql = format!("UPDATE email_config SET {} WHERE id = 1", sets.join(", "));
        let mut q = sqlx::query(&sql);

        if let Some(v) = &patch.smtp_server { q = q.bind(v); }
        if let Some(v) = patch.port         { q = q.bind(v as i64); }
        if let Some(v) = patch.tls          { q = q.bind(v as i64); }
        if let Some(v) = &patch.username    { q = q.bind(v); }
        if let Some(v) = &patch.password    { q = q.bind(v); }
        if let Some(v) = &patch.fromname    { q = q.bind(v); }

        q.execute(&self.pool).await?;
        Ok(())
    }
}
