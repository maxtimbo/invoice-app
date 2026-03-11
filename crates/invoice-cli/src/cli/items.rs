use anyhow::{anyhow, Result};
use clap::{Args, Subcommand};
use inquire::{Confirm, Select, Text};
use rust_decimal::Decimal;
use std::str::FromStr;

use invoice_app::ports::repos::item_repo::{CreateItem, ItemRepo, UpdateItem};
use invoice_core::models::{item::Item, currency::Currency, ids::ItemId};
use invoice_storage::sqlite::SqliteStorage;
use invoice_cli::{resolve_id, prompt_id};

#[derive(Args)]
pub struct ItemsArgs {
    #[command(subcommand)]
    pub command: Option<ItemsCommand>,
}

#[derive(Subcommand)]
pub enum ItemsCommand {
    /// List all items and view details
    List,
    /// Add a new item
    Add,
    /// Update an existing item
    Update { id: Option<i64> },
    /// Delete an item
    Delete { id: Option<i64> },
}

pub async fn run(args: ItemsArgs, db: &SqliteStorage) -> Result<()> {
    match args.command {
        Some(ItemsCommand::List)            => list(db).await,
        Some(ItemsCommand::Add)             => add(db).await,
        Some(ItemsCommand::Update { id })   => {
            let id = resolve_id!(id, db, list_item, Item, ItemId,
                "No items found", "Select item:");
            update(id, db).await
        }
        Some(ItemsCommand::Delete { id })   => {
            let id = resolve_id!(id, db, list_item, Item, ItemId,
                "No items found", "Select item:");
            delete(id, db).await
        }
        None => interactive(db).await,
    }
}

pub async fn interactive(db: &SqliteStorage) -> Result<()> {
    let choice = Select::new(
        "Items →",
        vec!["List", "Add", "Update", "Delete", "Back"],
    )
    .prompt()?;

    match choice {
        "List"   => list(db).await,
        "Add"    => add(db).await,
        "Update" => update(ItemId(prompt_id("Item ID:")?), db).await,
        "Delete" => delete(ItemId(prompt_id("Item ID:")?), db).await,
        _        => Ok(()),
    }
}

async fn list(db: &SqliteStorage) -> Result<()> {
    let all = db.list_item().await?;
    if all.is_empty() {
        println!("No items found.");
        return Ok(());
    }

    let choice = Select::new("Select an item to view:", all).prompt()?;
    println!("{}", choice);
    Ok(())
}

async fn add(db: &SqliteStorage) -> Result<()> {
    let name = Text::new("Name:").prompt()?;
    let rate = prompt_rate("Rate (e.g. 100.00):", None)?;

    let id = db.create_item(CreateItem { name, rate }).await?;
    println!("Created item #{}.", id.0);
    Ok(())
}

async fn update(id: ItemId, db: &SqliteStorage) -> Result<()> {
    let existing = db
        .get_item(id)
        .await?
        .ok_or_else(|| anyhow!("Item #{} not found.", id.0))?;

    println!("Current: {}\n---", existing);

    let new_name = {
        let input = Text::new("Name:").with_default(&existing.name).prompt()?;
        if input == existing.name { None } else { Some(input) }
    };

    let new_rate = {
        let current_str = existing.rate.0.to_string();
        let input = Text::new("Rate:").with_default(&current_str).prompt()?;
        let parsed = parse_rate(&input)?;
        if parsed.0 == existing.rate.0 { None } else { Some(parsed) }
    };

    db.update_item(id, UpdateItem { name: new_name, rate: new_rate }).await?;
    println!("Updated item #{}.", id.0);
    Ok(())
}

async fn delete(id: ItemId, db: &SqliteStorage) -> Result<()> {
    let existing = db
        .get_item(id)
        .await?
        .ok_or_else(|| anyhow!("Item #{} not found.", id.0))?;

    let confirmed = Confirm::new(&format!(
        "Delete item '{}' (#{})? This cannot be undone.",
        existing.name, id.0
    ))
    .with_default(false)
    .prompt()?;

    if confirmed {
        db.delete_item(id).await?;
        println!("Deleted item #{}.", id.0);
    } else {
        println!("Cancelled.");
    }
    Ok(())
}

fn parse_rate(input: &str) -> Result<Currency> {
    let d = Decimal::from_str(input.trim())
        .map_err(|_| anyhow!("'{}' is not a valid decimal amount.", input))?;
    if d < Decimal::ZERO {
        return Err(anyhow!("Rate cannot be negative."));
    }
    Ok(Currency::new(d))
}

fn prompt_rate(label: &str, default: Option<&Currency>) -> Result<Currency> {
    let default_str = default.map(|c| c.0.to_string()).unwrap_or_default();
    let mut builder = Text::new(label);
    if !default_str.is_empty() {
        builder = builder.with_default(&default_str);
    }
    let input = builder.prompt()?;
    parse_rate(&input)
}
