use anyhow::Result;
use clap::{Args, Subcommand};
use invoice_storage::sqlite::SqliteStorage;

#[derive(Args)]
pub struct InvoiceArgs {
    #[command(subcommand)]
    pub command: Option<InvoiceCommand>,
}

#[derive(Subcommand)]
pub enum InvoiceCommand {
    /// List invoice summaries
    List,
    /// Create a new invoice
    Add,
    /// Update invoice status, stage, or notes
    Update { id: i64 },
    /// Delete an invoice
    Delete { id: i64 },
    /// Render an invoice to HTML
    Render { id: i64 },
    /// Render an invoice to PDF
    Pdf { id: i64 },
    /// Email an invoice
    Email { id: i64 },
}

pub async fn run(args: InvoiceArgs, db: &SqliteStorage) -> Result<()> {
    // TODO: implement commands following company.rs pattern
    println!("Invoice management not yet implemented.");
    Ok(())
}

pub async fn interactive(db: &SqliteStorage) -> Result<()> {
    println!("Invoice management not yet implemented.");
    Ok(())
}
