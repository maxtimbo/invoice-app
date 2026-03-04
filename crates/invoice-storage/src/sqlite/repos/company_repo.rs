
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use sqlx::Row;

use crate::sqlite::SqliteStorage;
use invoice_app::ports::repos::company_repo::{
    CompanyRepo,
    CreateCompany,
    UpdateCompany
};
use invoice_core::models::{
    company::Company,
    contact::Contact,
    ids::CompanyId,
};

#[async_trait]
impl CompanyRepo for SqliteStorage {
    async fn get(&self, id: CompanyId) -> Result<Option<Company>> {
        let row = sqlx::query(
            "SELECT id, name, logo, phone, email, addr1, addr2, city, state, zip
             FROM company WHERE id = ?"
        )
        .bind(id.0)
        .fetch_optional(&self.pool)
        .await?;
    
        Ok(row.map(|r| Company {
            id: CompanyId(r.get::<i64, _>("id")),
            name: r.get::<String, _>("name"),
            logo: r.get::<Option<Vec<u8>>, _>("logo"),
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
    async fn list(&self) -> Result<Vec<Company>> {
        let rows = sqlx::query(
            "SELECT id, name, logo, phone, email, addr1, addr2, city, state, zip
            FROM company ORDER BY id")
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter()
            .map(|r| Company {
                id: CompanyId(r.get::<i64, _>("id")),
                name: r.get::<String, _>("name"),
                logo: r.get::<Option<Vec<u8>>, _>("logo"),
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
    async fn create(&self, input: CreateCompany) -> Result<CompanyId> {
        let c = input.contact;

        let res = sqlx::query(
            "INSERT INTO company
            (name, logo, phone, email, addr1, addr2, city, state, zip)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(input.name)
            .bind(input.logo)
            .bind(c.phone)
            .bind(c.email)
            .bind(c.addr1)
            .bind(c.addr2)
            .bind(c.city)
            .bind(c.state)
            .bind(c.zip)
            .execute(&self.pool)
            .await?;
        Ok(CompanyId(res.last_insert_rowid()))
    }
    async fn update(&self, id: CompanyId, patch: UpdateCompany) -> Result<()> {
        let mut company = self.get(id).await?.ok_or_else(|| anyhow!("Company {} not found", id.0))?;

        if let Some(name) = patch.name {
            company.name = name;
        }

        if let Some(contact) = patch.contact {
            company.contact = contact;
        }

        let c = company.contact;

        sqlx::query(
            "UPDATE company SET
            name = ?,
            logo = ?,
            phone = ?,
            email = ?,
            addr1 = ?,
            addr2 = ?,
            city = ?,
            state = ?,
            zip = ?
            WHERE id = ?")
            .bind(company.name)
            .bind(company.logo)
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
    async fn delete(&self, id: CompanyId) -> Result<bool> {
        let res = sqlx::query("DELETE FROM company WHERE id = ?")
            .bind(id.0)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected() > 0)
    }
}
