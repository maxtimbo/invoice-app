
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use sqlx::Row;

use crate::sqlite::SqliteStorage;
use invoice_app::ports::repos::client_repo::{
    ClientRepo,
    CreateClient,
    UpdateClient
};
use invoice_core::models::{
    client::Client,
    contact::Contact,
    ids::ClientId,
};


#[async_trait]
impl ClientRepo for SqliteStorage {
    async fn get_client(&self, id: ClientId) -> Result<Option<Client>> {
        let row = sqlx::query(
            "SELECT id, name, phone, email, addr1, addr2, city, state, zip
             FROM client WHERE id = ?"
        )
        .bind(id.0)
        .fetch_optional(&self.pool)
        .await?;
    
        Ok(row.map(|r| Client {
            id: ClientId(r.get::<i64, _>("id")),
            name: r.get::<String, _>("name"),
            contact: Contact {
                phone: r.get::<Option<String>, _>("phone"),
                email: r.get::<Option<String>, _>("email"),
                addr1: r.get::<Option<String>, _>("addr1"),
                addr2: r.get::<Option<String>, _>("addr2"),
                city:  r.get::<Option<String>, _>("city"),
                state: r.get::<Option<String>, _>("state"),
                zip:   r.get::<Option<String>, _>("zip"),
            },
        }))
    }
    async fn list_client(&self) -> Result<Vec<Client>> {
        let rows = sqlx::query(
            "SELECT id, name, phone, email, addr1, addr2, city, state, zip
            FROM client ORDER BY id")
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter()
            .map(|r| Client {
                id: ClientId(r.get::<i64, _>("id")),
                name: r.get::<String, _>("name"),
                contact: Contact {
                    phone: r.get::<Option<String>, _>("phone"),
                    email: r.get::<Option<String>, _>("email"),
                    addr1: r.get::<Option<String>, _>("addr1"),
                    addr2: r.get::<Option<String>, _>("addr2"),
                    city:  r.get::<Option<String>, _>("city"),
                    state: r.get::<Option<String>, _>("state"),
                    zip:   r.get::<Option<String>, _>("zip"),
                },
            })
            .collect())
    }
    async fn create_client(&self, input: CreateClient) -> Result<ClientId> {
        let c = input.contact;

        let res = sqlx::query(
            "INSERT INTO client
            (name, phone, email, addr1, addr2, city, state, zip)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(input.name)
            .bind(c.phone)
            .bind(c.email)
            .bind(c.addr1)
            .bind(c.addr2)
            .bind(c.city)
            .bind(c.state)
            .bind(c.zip)
            .execute(&self.pool)
            .await?;
        Ok(ClientId(res.last_insert_rowid()))
    }
    async fn update_client(&self, id: ClientId, patch: UpdateClient) -> Result<()> {
        let mut client = self.get_client(id).await?.ok_or_else(|| anyhow!("Client {} not found", id.0))?;

        if let Some(name) = patch.name {
            client.name = name;
        }

        if let Some(contact) = patch.contact {
            client.contact = contact;
        }

        let c = client.contact;

        sqlx::query(
            "UPDATE client SET
            name = ?,
            phone = ?,
            email = ?,
            addr1 = ?,
            addr2 = ?,
            city = ?,
            state = ?,
            zip = ?
            WHERE id = ?")
            .bind(client.name)
            .bind(c.phone)
            .bind(c.email)
            .bind(c.addr1)
            .bind(c.addr2)
            .bind(c.city)
            .bind(c.state)
            .bind(c.zip)
            .bind(id.0)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    async fn delete_client(&self, id: ClientId) -> Result<bool> {
        let res = sqlx::query("DELETE FROM client WHERE id = ?")
            .bind(id.0)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected() > 0)
    }
}
