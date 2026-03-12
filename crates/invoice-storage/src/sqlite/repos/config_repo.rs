use async_trait::async_trait;
use anyhow::Result;
use sqlx::Row;

use invoice_app::ports::repos::config_repo::{
    ConfigRepo,
    UpsertConfig
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
                fromname,
                test_recipient
            FROM email_config WHERE id = 0",
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
            test_recipient: r.get::<Option<String>, _>("test_recipient"),
        }))
    }

    async fn upsert_config(&self, input: UpsertConfig) -> Result<()> {
        sqlx::query(
            "INSERT OR REPLACE INTO email_config (
                id,
                smtp_server,
                port,
                tls,
                username,
                password,
                fromname,
                test_recipient)
            VALUES (0, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&input.smtp_server)
        .bind(input.port as i64)
        .bind(input.tls as i64)
        .bind(&input.username)
        .bind(&input.password)
        .bind(&input.fromname)
        .bind(input.test_recipient)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
