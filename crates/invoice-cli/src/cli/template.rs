use anyhow::Result;
use clap::{Args, Subcommand};
use invoice_storage::sqlite::SqliteStorage;

#[derive(Args)]
pub struct TemplateArgs {
    #[command(subcommand)]
    pub command: Option<TemplateCommand>,
}

#[derive(Subcommand)]
pub enum TemplateCommand {
    /// List all templates
    List,
    /// Show a template by ID
    Get { id: i64 },
    /// Add a new template
    Add,
    /// Update an existing template
    Update { id: i64 },
    /// Delete a template
    Delete { id: i64 },
}

pub async fn run(args: TemplateArgs, db: &SqliteStorage) -> Result<()> {
    println!("Template management not yet implemented.");
    Ok(())
}

pub async fn interactive(db: &SqliteStorage) -> Result<()> {
    println!("Template management not yet implemented.");
    Ok(())
}
