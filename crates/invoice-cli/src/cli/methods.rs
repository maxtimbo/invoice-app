use anyhow::Result;
use clap::{Args, Subcommand};
use invoice_storage::sqlite::SqliteStorage;

#[derive(Args)]
pub struct MethodsArgs {
    #[command(subcommand)]
    pub command: Option<MethodsCommand>,
}

#[derive(Subcommand)]
pub enum MethodsCommand {
    /// List all payment methods
    List,
    /// Show a method by ID
    Add,
    /// Update an existing method
    Update { id: i64 },
    /// Delete a payment method
    Delete { id: i64 },
}

pub async fn run(args: MethodsArgs, db: &SqliteStorage) -> Result<()> {
    println!("Methods management not yet implemented.");
    Ok(())
}

pub async fn interactive(db: &SqliteStorage) -> Result<()> {
    println!("Methods management not yet implemented.");
    Ok(())
}
