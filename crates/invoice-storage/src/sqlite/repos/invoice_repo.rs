use anyhow::{anyhow, Result};
use std::str::FromStr;
use async_trait::async_trait;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::Row;

use crate::sqlite::SqliteStorage;
use crate::sqlite::models::invoice_skel::{InvoiceSkel, InvoiceItemSkel};
use invoice_app::ports::repos::invoice_repo::{CreateInvoice, UpdateInvoice, InvoiceRepo};
use invoice_app::ports::repos::template_repo::TemplateRepo;
use invoice_app::ports::repos::item_repo::ItemRepo;
use invoice_core::models::{
    ids::{InvoiceId, ItemId, TemplateId},
    invoice::Invoice,
    summary::InvoiceSummary,
    attributes::InvoiceAttrs,
    stage::InvoiceStage,
    status::PaidStatus,
    quantity::Quantity,
    currency::Currency,
    item::Item,
};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct ItemRecord {
    item_id: i64,
    quantity: String,
}

impl SqliteStorage {
    async fn fetch_invoice_skel(&self, id: InvoiceId) -> Result<Option<InvoiceSkel>> {
        let row = sqlx::query(
            "SELECT
                id,
                template_id,
                date,
                show_methods,
                show_notes,
                stage,
                status,
                status_date,
                status_check,
                notes,
                items_json,
                total
            FROM invoices WHERE id = ?")
            .bind(id.0)
            .fetch_optional(&self.pool)
            .await?;

        row.map(|r| self.row_to_skel(r)).transpose()
    }

    fn row_to_skel(&self, r: sqlx::sqlite::SqliteRow) -> Result<InvoiceSkel> {
        let date_str: String = r.get::<String, _>("date");
        let date = NaiveDate::parse_from_str(&date_str, "%Y%m%d")
            .map_err(|e| anyhow!("invalid date: {}", e))?;

        let stage = match r.get::<String, _>("stage").as_str() {
            "Quote" => InvoiceStage::Quote,
            "Invoice" => InvoiceStage::Invoice,
            s => return Err(anyhow!("unknown stage: {}", s)),
        };

        let status_str: String = r.get::<String, _>("status");
        let status_date: Option<String> = r.get("status_date");
        let status_check: Option<String> = r.get("status_check");

        let status = match status_str.as_str() {
            "Waiting" => PaidStatus::Waiting,
            "Past Due" => PaidStatus::PastDue,
            "Paid" => {
                let date = NaiveDate::parse_from_str(
                    &status_date.ok_or_else(|| anyhow!("Paid status missing date"))?,
                    "%Y%m%d"
                )?;
                PaidStatus::Paid { date, check: status_check }
            }
            "Failed" => {
                let date = NaiveDate::parse_from_str(
                    &status_date.ok_or_else(|| anyhow!("Failed status missing date"))?,
                    "%Y%m%d"
                )?;
                PaidStatus::Failed { date }
            }
            "Refunded" => {
                let date = NaiveDate::parse_from_str(
                    &status_date.ok_or_else(|| anyhow!("Refunded status missing date"))?,
                    "%Y%m%d"
                )?;
                PaidStatus::Refunded { date }
            }
            s => return Err(anyhow!("unknown status: {}", s)),
        };

        let items_json: String = r.get::<String, _>("items_json");
        let item_records: Vec<ItemRecord> = serde_json::from_str(&items_json)
            .map_err(|e| anyhow!("invalis items_json: {}", e))?;
        let items = item_records
            .into_iter()
            .map(|rec| {
                let qty_dec = Decimal::from_str(&rec.quantity)
                    .map_err(|e| anyhow!("invalid quantity:{}", e))?;
                Ok(InvoiceItemSkel {
                    item_id: ItemId(rec.item_id),
                    quantity: Quantity::new(qty_dec)?,
                })
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(InvoiceSkel {
            id: InvoiceId(r.get::<i64, _>("id")),
            template_id: TemplateId(r.get::<i64, _>("template_id")),
            date,
            attributes: InvoiceAttrs {
                show_methods: r.get::<bool, _>("show_methods"),
                show_notes: r.get::<bool, _>("show_notes"),
                stage,
                status,
            },
            notes: r.get("notes"),
            items,
        })
    }
    async fn hydrate_invoice(&self, skel: InvoiceSkel) -> Result<Invoice> {
        let template = self.get_template(skel.template_id).await?
            .ok_or_else(|| anyhow!("template {} not found", skel.template_id.0))?;

        let mut items: HashMap<Item, Quantity> = HashMap::new();

        for item_skel in skel.items {
            let item = self.get_item(item_skel.item_id).await?
                .ok_or_else(|| anyhow!("item {} not found", item_skel.item_id.0))?;
            items.insert(item, item_skel.quantity);
        }

        Ok(Invoice {
            id: skel.id,
            template,
            date: skel.date,
            attributes: skel.attributes,
            notes: skel.notes,
            items,
        })
    }
    fn compute_total(items: &[(Item, Quantity)]) -> i64 {
        let total: Decimal = items
            .iter()
            .map(|(item, qty)| item.rate.inner() * qty.inner())
            .sum();
        Currency::new(total).to_cents()
    }
}

#[async_trait]
impl InvoiceRepo for SqliteStorage {
    async fn get_invoice(&self, id: InvoiceId) -> Result<Option<Invoice>> {
        match self.fetch_invoice_skel(id).await? {
            Some(skel) => Ok(Some(self.hydrate_invoice(skel).await?)),
            None => Ok(None),
        }
    }

    async fn list_invoice_summary(&self) -> Result<Vec<InvoiceSummary>> {
        let rows = sqlx::query(
            "SELECT
                i.id,
                i.date,
                i.status,
                i.status_date,
                i.total,
                c.name as client_name,
                t.due as terms_due
            FROM invoices i
            JOIN templates tmpl ON tmpl.id = i.template_id
            JOIN client c ON c.id = tmpl.client_id
            JOIN terms t on t.id = tmpl.terms_id
            ORDER BY i.date DESC")
            .fetch_all(&self.pool)
            .await?;

        let mut summaries = Vec::new();
        for r in rows {
            let date = NaiveDate::parse_from_str(
                &r.get::<String, _>("date"), "%Y%m%d"
            )?;

            let terms_due: i64 = r.get("terms_due");
            let due_date = date + chrono::Duration::days(terms_due);

            let status_str: String = r.get("status");
            let status_date: Option<String> = r.get("status_date");
            let status = match status_str.as_str() {
                "Waiting" => PaidStatus::Waiting,
                "Past Due" => PaidStatus::PastDue,
                "Paid" => PaidStatus::Paid {
                    date: NaiveDate::parse_from_str(
                              &status_date.ok_or_else(|| anyhow!("Paid missing date"))?,
                              "%Y%m%d"
                          )?,
                        check: None,
                },
                "Failed" => PaidStatus::Failed {
                    date: NaiveDate::parse_from_str(
                              &status_date.ok_or_else(|| anyhow!("Failed missing date"))?,
                              "%Y%m%d"
                          )?,
                },
                "Refunded" => PaidStatus::Refunded {
                    date: NaiveDate::parse_from_str(
                              &status_date.ok_or_else(|| anyhow!("Refunded missing date"))?,
                              "%Y%m%d"
                          )?,
                },
                s => return Err(anyhow!("unknown status: {}", s)),
            };

            let total = Currency::from_cents(r.get::<i64, _>("total"));
            summaries.push(InvoiceSummary {
                id: InvoiceId(r.get::<i64, _>("id")),
                issued: date,
                client_name: r.get("client_name"),
                total,
                status,
                due: due_date,
            });
        }
        Ok(summaries)
    }

    async fn create_invoice(&self, input: CreateInvoice) -> Result<InvoiceId> {
        let mut hydrated_items = Vec::new();
        for (item_id, quantity) in &input.items {
            let item = self.get_item(*item_id).await?
                .ok_or_else(|| anyhow!("item {} not found", item_id.0))?;
            hydrated_items.push((item, quantity.clone()));
        }

        let total = Self::compute_total(&hydrated_items);

        let items_json = serde_json::to_string(
            &input.items.iter().map(|(id, qty)| ItemRecord {
                item_id: id.0,
                quantity: qty.inner().to_string(),
            }).collect::<Vec<_>>()
        )?;

        let (status_str, status_date, status_check) = encode_status(&input.attributes.status);
        let stage_str = encode_stage(&input.attributes.stage);

        let id = sqlx::query(
            "INSERT INTO invoices
                (template_id, date, show_methods, show_notes, stage,
                 status, status_date, status_check, notes, items_json, total)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(input.template.0)
            .bind(input.date.format("%Y%m%d").to_string())
            .bind(input.attributes.show_methods)
            .bind(input.attributes.show_notes)
            .bind(stage_str)
            .bind(status_str)
            .bind(status_date)
            .bind(status_check)
            .bind(input.notes)
            .bind(items_json)
            .bind(total)
            .execute(&self.pool)
            .await?
            .last_insert_rowid();

        Ok(InvoiceId(id))
    }

    async fn update_invoice(&self, id: InvoiceId, patch: UpdateInvoice) -> Result<()> {
        let mut skel = self.fetch_invoice_skel(id).await?
            .ok_or_else(|| anyhow!("invoice {} not found", id.0))?;

        if let Some(v) = patch.show_methods { skel.attributes.show_methods = v; }
        if let Some(v) = patch.show_notes { skel.attributes.show_notes = v; }
        if let Some(v) = patch.stage { skel.attributes.stage = v; }
        if let Some(v) = patch.status { skel.attributes.status = v; }
        if let Some(v) = patch.notes { skel.notes = Some(v); }

        let (status_str, status_date, status_check) = encode_status(&skel.attributes.status);
        let stage_str = encode_stage(&skel.attributes.stage);

        sqlx::query(
            "UPDATE invoices
            SET
                show_methods = ?,
                show_notes = ?,
                stage = ?,
                status = ?,
                status_date = ?,
                status_check = ?,
                notes = ?
            WHERE id = ?")
            .bind(skel.attributes.show_methods)
            .bind(skel.attributes.show_notes)
            .bind(stage_str)
            .bind(status_str)
            .bind(status_date)
            .bind(status_check)
            .bind(skel.notes)
            .bind(id.0)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn delete_invoice(&self, id: InvoiceId) -> Result<bool> {
        let res = sqlx::query("DELETE FROM invoices WHERE id = ?")
            .bind(id.0)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected() > 0)
    }
}

fn encode_status(status: &PaidStatus) -> (String, Option<String>, Option<String>) {
    match status {
        PaidStatus::Waiting => ("Waiting".into(), None, None),
        PaidStatus::PastDue => ("Past Due".into(), None, None),
        PaidStatus::Paid { date, check } => (
            "Paid".into(),
            Some(date.format("%Y%m%d").to_string()),
            check.clone(),
        ),
        PaidStatus::Failed { date } => (
            "Failed".into(),
            Some(date.format("%Y%m%d").to_string()),
            None,
        ),
        PaidStatus::Refunded { date } => (
            "Refunded".into(),
            Some(date.format("%Y%m%d").to_string()),
            None,
        ),
    }
}

fn encode_stage(stage: &InvoiceStage) -> &'static str {
    match stage {
        InvoiceStage::Quote => "Quote",
        InvoiceStage::Invoice => "Invoice",
    }
}
