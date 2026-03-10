use anyhow::Result;
use clap::{Args, Subcommand};
use invoice_storage::sqlite::SqliteStorage;

#[derive(Args)]
pub struct ItemsArgs {
    #[command(subcommand)]
    pub command: Option<ItemsCommand>,
}

#[derive(Subcommand)]
pub enum ItemsCommand {
    /// List all items
    List,
    /// Add a new item
    Add,
    /// Update an existing item
    Update { id: i64 },
    /// Delete an item
    Delete { id: i64 },
}

pub async fn run(args: ItemsArgs, db: &SqliteStorage) -> Result<()> {
    // TODO: mirror company.rs, using ItemRepo + CreateItem + UpdateItem
    // Rate prompting: Text::new("Rate (e.g. 100.00):"), parse with Decimal::from_str,
    // wrap in Currency::new()
    println!("Items management not yet implemented.");
    Ok(())
}

pub async fn interactive(db: &SqliteStorage) -> Result<()> {
    println!("Items management not yet implemented.");
    Ok(())
}
