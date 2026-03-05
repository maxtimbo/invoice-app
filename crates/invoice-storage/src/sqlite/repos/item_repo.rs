use anyhow::{anyhow, Result};
use async_trait::async_trait;
use sqlx::Row;

use crate::sqlite::SqliteStorage;
use invoice_app::ports::repos::item_repo::{
    ItemRepo,
    CreateItem,
    UpdateItem
};
use invoice_core::models::{
    item::Item,
    ids::ItemId,
    currency::Currency
};

#[async_trait]
impl ItemRepo for SqliteStorage {
    async fn get_item(&self, id: ItemId) -> Result<Option<Item>> {
        let row = sqlx::query("SELECT id, name, rate FROM items WHERE id = ?")
            .bind(id.0)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|r| Item {
            id: ItemId(r.get::<i64, _>("id")),
            name: r.get::<String, _>("name"),
            rate: Currency::from_cents(r.get::<i64, _>("rate")),
        }))
    }
    async fn list_item(&self) -> Result<Vec<Item>> {
        let rows = sqlx::query("SELECT id, name, rate FROM items ORDER BY name")
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter()
            .map(|r| Item {
                id: ItemId(r.get::<i64, _>("id")),
                name: r.get::<String, _>("name"),
                rate: Currency::from_cents(r.get::<i64, _>("rate")),
            })
            .collect())
    }
    async fn create_item(&self, input: CreateItem) -> Result<ItemId> {
        let res = sqlx::query("INSERT INTO items (name, rate) VALUES (?, ?)")
            .bind(input.name)
            .bind(input.rate.to_cents())
            .execute(&self.pool)
            .await?;
        Ok(ItemId(res.last_insert_rowid()))
    }
    async fn update_item(&self, id: ItemId, patch: UpdateItem) -> Result<()> {
        let mut item = self.get_item(id).await?.ok_or_else(|| anyhow!("item {} not found", id.0))?;
        if let Some(name) = patch.name {
            item.name = name;
        }
        if let Some(rate) = patch.rate {
            item.rate = rate;
        }
        sqlx::query("UPDATE items SET name = ?, rate = ? WHERE id = ?")
            .bind(item.name)
            .bind(item.rate.to_cents())
            .bind(id.0)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    async fn delete_item(&self, id: ItemId) -> Result<bool> {
        let res = sqlx::query("DELETE FROM items WHERE id = ?")
            .bind(id.0)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected() > 0)
    }
}

