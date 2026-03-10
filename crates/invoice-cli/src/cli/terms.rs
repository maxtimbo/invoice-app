use anyhow::Result;
use clap::{Args, Subcommand};
use invoice_storage::sqlite::SqliteStorage;

#[derive(Args)]
pub struct TermsArgs {
    #[command(subcommand)]
    pub command: Option<TermsCommand>,
}

#[derive(Subcommand)]
pub enum TermsCommand {
    /// List all payment terms
    List,
    /// Show terms by ID
    Add,
    /// Update existing terms
    Update { id: i64 },
    /// Delete payment terms
    Delete { id: i64 },
}

pub async fn run(args: TermsArgs, db: &SqliteStorage) -> Result<()> {
    // TODO: mirror company.rs, using TermsRepo + CreateTerms + UpdateTerms
    // due field: Text::new("Due (days):"), parse as i64
    println!("Terms management not yet implemented.");
    Ok(())
}

pub async fn interactive(db: &SqliteStorage) -> Result<()> {
    println!("Terms management not yet implemented.");
    Ok(())
}
