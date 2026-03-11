use anyhow::{anyhow, Result};
use clap::{Args, Subcommand};
use inquire::{Confirm, Select, Text};

use invoice_app::ports::repos::method_repo::{CreateMethod, MethodRepo, UpdateMethod};
use invoice_core::models::ids::MethodId;
use invoice_storage::sqlite::SqliteStorage;
use invoice_cli::{prompt_id, prompt_image, prompt_optional};

#[derive(Args)]
pub struct MethodsArgs {
    #[command(subcommand)]
    pub command: Option<MethodsCommand>,
}

#[derive(Subcommand)]
pub enum MethodsCommand {
    /// List all payment methods and view details
    List,
    /// Add a new payment method
    Add,
    /// Update an existing payment method
    Update { id: i64 },
    /// Delete a payment method
    Delete { id: i64 },
}

pub async fn run(args: MethodsArgs, db: &SqliteStorage) -> Result<()> {
    match args.command {
        Some(MethodsCommand::List)            => list(db).await,
        Some(MethodsCommand::Add)             => add(db).await,
        Some(MethodsCommand::Update { id })   => update(MethodId(id), db).await,
        Some(MethodsCommand::Delete { id })   => delete(MethodId(id), db).await,
        None => interactive(db).await,
    }
}

pub async fn interactive(db: &SqliteStorage) -> Result<()> {
    let choice = Select::new(
        "Methods →",
        vec!["List", "Add", "Update", "Delete", "Back"],
    )
    .prompt()?;

    match choice {
        "List"   => list(db).await,
        "Add"    => add(db).await,
        "Update" => update(MethodId(prompt_id("Method ID:")?), db).await,
        "Delete" => delete(MethodId(prompt_id("Method ID:")?), db).await,
        _        => Ok(()),
    }
}

async fn list(db: &SqliteStorage) -> Result<()> {
    let all = db.list_method().await?;
    if all.is_empty() {
        println!("No payment methods found.");
        return Ok(());
    }

    let choice = Select::new("Select a method to view:", all).prompt()?;
    println!("{}", choice);
    Ok(())
}

async fn add(db: &SqliteStorage) -> Result<()> {
    let name = Text::new("Name:").prompt()?;
    let link = prompt_optional("Link (leave blank to skip):", "")?;
    let qr   = prompt_image("QR code path (leave blank to skip):")?;

    let id = db.create_method(CreateMethod { name, link, qr }).await?;
    println!("Created method #{}.", id.0);
    Ok(())
}

async fn update(id: MethodId, db: &SqliteStorage) -> Result<()> {
    let existing = db
        .get_method(id)
        .await?
        .ok_or_else(|| anyhow!("Method #{} not found.", id.0))?;

    println!("Current:\n{}\n---", existing);

    let new_name = {
        let input = Text::new("Name:").with_default(&existing.name).prompt()?;
        if input == existing.name { None } else { Some(input) }
    };

    let new_link = if Confirm::new("Update link?").with_default(false).prompt()? {
        prompt_optional("Link (leave blank to clear):", existing.link.as_deref().unwrap_or(""))?
    } else {
        existing.link
    };

    let new_qr = if Confirm::new("Update QR code?").with_default(false).prompt()? {
        prompt_image("QR code path (leave blank to clear):")?
    } else {
        existing.qr
    };

    db.update_method(id, UpdateMethod {
        name: new_name,
        link: new_link,
        qr:   new_qr,
    })
    .await?;
    println!("Updated method #{}.", id.0);
    Ok(())
}

async fn delete(id: MethodId, db: &SqliteStorage) -> Result<()> {
    let existing = db
        .get_method(id)
        .await?
        .ok_or_else(|| anyhow!("Method #{} not found.", id.0))?;

    let confirmed = Confirm::new(&format!(
        "Delete method '{}' (#{})? This cannot be undone.",
        existing.name, id.0
    ))
    .with_default(false)
    .prompt()?;

    if confirmed {
        db.delete_method(id).await?;
        println!("Deleted method #{}.", id.0);
    } else {
        println!("Cancelled.");
    }
    Ok(())
}
