use anyhow::{anyhow, Result};
use clap::{Args, Subcommand};
use chrono::NaiveDate;
use inquire::{Confirm, DateSelect, MultiSelect, Select, Text};
use rust_decimal::Decimal;
use std::str::FromStr;

use invoice_app::ports::repos::{
    invoice_repo::{CreateInvoice, InvoiceRepo, UpdateInvoice},
    template_repo::TemplateRepo,
    item_repo::ItemRepo,
};
use invoice_core::models::{
    attributes::InvoiceAttrs,
    ids::InvoiceId,
    quantity::Quantity,
    stage::InvoiceStage,
    status::PaidStatus,
};
use invoice_storage::sqlite::SqliteStorage;
use invoice_cli::{editor_optional, prompt_id};
use invoice_cli::render::{render_html, render_pdf};

use super::email;

#[derive(Args)]
pub struct InvoiceArgs {
    #[command(subcommand)]
    pub command: Option<InvoiceCommand>,
}

#[derive(Subcommand)]
pub enum InvoiceCommand {
    /// List invoices and view details
    List,
    /// Create a new invoice
    Add,
    /// Update invoice status, stage, or notes
    Update { id: i64 },
    /// Delete an invoice
    Delete { id: i64 },
    /// Render invoice to HTML
    Render { id: i64 },
    /// Render invoice to PDF
    Pdf { id: i64 },
    /// Email an invoice
    Email { id: i64 },
}

pub async fn run(args: InvoiceArgs, db: &SqliteStorage) -> Result<()> {
    match args.command {
        Some(InvoiceCommand::List)           => list(db).await,
        Some(InvoiceCommand::Add)            => add(db).await,
        Some(InvoiceCommand::Update { id })  => update(InvoiceId(id), db).await,
        Some(InvoiceCommand::Delete { id })  => delete(InvoiceId(id), db).await,
        Some(InvoiceCommand::Render { id })  => render(InvoiceId(id), db).await,
        Some(InvoiceCommand::Pdf { id })     => pdf(InvoiceId(id), db).await,
        Some(InvoiceCommand::Email { id })   => email::send(db, id).await,
        None => interactive(db).await,
    }
}

pub async fn interactive(db: &SqliteStorage) -> Result<()> {
    let choice = Select::new(
        "Invoices →",
        vec!["List", "Add", "Update", "Delete", "Render", "PDF", "Email", "Back"],
    )
    .prompt()?;

    match choice {
        "List"   => list(db).await,
        "Add"    => add(db).await,
        "Update" => update(InvoiceId(prompt_id("Invoice ID:")?), db).await,
        "Delete" => delete(InvoiceId(prompt_id("Invoice ID:")?), db).await,
        "Render" => render(InvoiceId(prompt_id("Invoice ID:")?), db).await,
        "PDF"    => pdf(InvoiceId(prompt_id("Invoice ID:")?), db).await,
        "Email"  => {
            let id = prompt_id("Invoice ID:")?;
            email::send(db, id).await
        }
        _ => Ok(()),
    }
}

async fn list(db: &SqliteStorage) -> Result<()> {
    let all = db.list_invoice_summary().await?;
    if all.is_empty() {
        println!("No invoices found.");
        return Ok(());
    }

    let choice = Select::new("Select an invoice to view:", all).prompt()?;

    let full = db
        .get_invoice(choice.id)
        .await?
        .ok_or_else(|| anyhow!("Invoice #{} not found.", choice.id))?;

    println!("{}", full);
    Ok(())
}

async fn add(db: &SqliteStorage) -> Result<()> {
    let templates = db.list_template().await?;
    if templates.is_empty() {
        return Err(anyhow!("No templates found. Add one first."));
    }
    let template = Select::new("Template:", templates).prompt()?;

    let date: NaiveDate = DateSelect::new("Invoice date:").prompt()?;

    let stage = Select::new("Stage:", vec!["Invoice", "Quote"]).prompt()?;
    let stage = match stage {
        "Quote"   => InvoiceStage::Quote,
        _         => InvoiceStage::Invoice,
    };

    let show_methods = Confirm::new("Show payment methods?").with_default(true).prompt()?;
    let show_notes   = Confirm::new("Show notes section?").with_default(true).prompt()?;

    let notes = editor_optional("Notes (optional):", "")?;

    let all_items = db.list_item().await?;
    if all_items.is_empty() {
        return Err(anyhow!("No items found. Add one first."));
    }
    let selected_items = MultiSelect::new("Line items (space to select):", all_items).prompt()?;
    if selected_items.is_empty() {
        return Err(anyhow!("At least one item is required."));
    }

    let mut items = Vec::new();
    for item in &selected_items {
        let qty = prompt_quantity(&format!("Quantity for '{}':", item.name))?;
        items.push((item.id, qty));
    }

    let id = db.create_invoice(CreateInvoice {
        template: template.id,
        date,
        attributes: InvoiceAttrs {
            show_methods,
            show_notes,
            stage,
            status: PaidStatus::Waiting,
        },
        notes,
        items,
    })
    .await?;

    println!("Created invoice #{}.", id.0);
    Ok(())
}

async fn update(id: InvoiceId, db: &SqliteStorage) -> Result<()> {
    let existing = db
        .get_invoice(id)
        .await?
        .ok_or_else(|| anyhow!("Invoice #{} not found.", id.0))?;

    println!("Current:\n{}\n---", existing);

    let new_stage = if Confirm::new("Update stage?").with_default(false).prompt()? {
        let s = Select::new("Stage:", vec!["Invoice", "Quote"]).prompt()?;
        Some(match s {
            "Quote" => InvoiceStage::Quote,
            _       => InvoiceStage::Invoice,
        })
    } else {
        None
    };

    let new_status = if Confirm::new("Update payment status?").with_default(false).prompt()? {
        Some(prompt_status()?)
    } else {
        None
    };

    let new_show_methods = if Confirm::new("Update show payment methods?").with_default(false).prompt()? {
        Some(Confirm::new("Show payment methods?")
            .with_default(existing.attributes.show_methods)
            .prompt()?)
    } else {
        None
    };

    let new_show_notes = if Confirm::new("Update show notes?").with_default(false).prompt()? {
        Some(Confirm::new("Show notes?")
            .with_default(existing.attributes.show_notes)
            .prompt()?)
    } else {
        None
    };

    let new_notes = if Confirm::new("Update notes?").with_default(false).prompt()? {
        let current = existing.notes.as_deref().unwrap_or("");
        editor_optional("Notes:", current)?
    } else {
        None
    };

    db.update_invoice(id, UpdateInvoice {
        stage:        new_stage,
        status:       new_status,
        show_methods: new_show_methods,
        show_notes:   new_show_notes,
        notes:        new_notes,
        ..Default::default()
    })
    .await?;

    println!("Updated invoice #{}.", id.0);
    Ok(())
}

async fn delete(id: InvoiceId, db: &SqliteStorage) -> Result<()> {
    let existing = db
        .get_invoice(id)
        .await?
        .ok_or_else(|| anyhow!("Invoice #{} not found.", id.0))?;

    let confirmed = Confirm::new(&format!(
        "Delete invoice #{} ({})? This cannot be undone.",
        id.0, existing.date
    ))
    .with_default(false)
    .prompt()?;

    if confirmed {
        db.delete_invoice(id).await?;
        println!("Deleted invoice #{}.", id.0);
    } else {
        println!("Cancelled.");
    }
    Ok(())
}

async fn render(id: InvoiceId, db: &SqliteStorage) -> Result<()> {
    let invoice = db
        .get_invoice(id)
        .await?
        .ok_or_else(|| anyhow!("Invoice #{} not found.", id.0))?;

    let html = render_html(&invoice)?;
    let filename = format!("invoice-{:04}.html", id.0);
    std::fs::write(&filename, &html)?;
    println!("Written to {}", filename);
    Ok(())
}

async fn pdf(id: InvoiceId, db: &SqliteStorage) -> Result<()> {
    let invoice = db
        .get_invoice(id)
        .await?
        .ok_or_else(|| anyhow!("Invoice #{} not found.", id.0))?;

    let bytes = render_pdf(&invoice)?;
    let filename = format!("invoice-{:04}.pdf", id.0);
    std::fs::write(&filename, &bytes)?;
    println!("Written to {}", filename);
    Ok(())
}

fn prompt_quantity(label: &str) -> Result<Quantity> {
    let input = Text::new(label).with_default("1").prompt()?;
    let d = Decimal::from_str(input.trim())
        .map_err(|_| anyhow!("'{}' is not a valid quantity.", input))?;
    Quantity::new(d).map_err(|e| anyhow!("{}", e))
}

fn prompt_status() -> Result<PaidStatus> {
    let choice = Select::new(
        "Status:",
        vec!["Waiting", "Past Due", "Paid", "Failed", "Refunded"],
    )
    .prompt()?;

    match choice {
        "Waiting"  => Ok(PaidStatus::Waiting),
        "Past Due" => Ok(PaidStatus::PastDue),
        "Paid" => {
            let date: NaiveDate = DateSelect::new("Payment date:").prompt()?;
            let check = {
                let input = Text::new("Check number (leave blank if none):").prompt()?;
                if input.trim().is_empty() { None } else { Some(input) }
            };
            Ok(PaidStatus::Paid { date, check })
        }
        "Failed" => {
            let date: NaiveDate = DateSelect::new("Failed date:").prompt()?;
            Ok(PaidStatus::Failed { date })
        }
        "Refunded" => {
            let date: NaiveDate = DateSelect::new("Refunded date:").prompt()?;
            Ok(PaidStatus::Refunded { date })
        }
        _ => unreachable!(),
    }
}
