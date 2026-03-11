use anyhow::{anyhow, Result};
use clap::{Args, Subcommand};
use inquire::{Confirm, Select, Text};

use invoice_app::ports::repos::terms_repo::{CreateTerms, TermsRepo, UpdateTerms};
use invoice_core::models::ids::TermsId;
use invoice_storage::sqlite::SqliteStorage;
use invoice_cli::prompt_id;

#[derive(Args)]
pub struct TermsArgs {
    #[command(subcommand)]
    pub command: Option<TermsCommand>,
}

#[derive(Subcommand)]
pub enum TermsCommand {
    /// List all payment terms and view details
    List,
    /// Add new payment terms
    Add,
    /// Update existing payment terms
    Update { id: i64 },
    /// Delete payment terms
    Delete { id: i64 },
}

pub async fn run(args: TermsArgs, db: &SqliteStorage) -> Result<()> {
    match args.command {
        Some(TermsCommand::List)            => list(db).await,
        Some(TermsCommand::Add)             => add(db).await,
        Some(TermsCommand::Update { id })   => update(TermsId(id), db).await,
        Some(TermsCommand::Delete { id })   => delete(TermsId(id), db).await,
        None => interactive(db).await,
    }
}

pub async fn interactive(db: &SqliteStorage) -> Result<()> {
    let choice = Select::new(
        "Terms →",
        vec!["List", "Add", "Update", "Delete", "Back"],
    )
    .prompt()?;

    match choice {
        "List"   => list(db).await,
        "Add"    => add(db).await,
        "Update" => update(TermsId(prompt_id("Terms ID:")?), db).await,
        "Delete" => delete(TermsId(prompt_id("Terms ID:")?), db).await,
        _        => Ok(()),
    }
}

async fn list(db: &SqliteStorage) -> Result<()> {
    let all = db.list_terms().await?;
    if all.is_empty() {
        println!("No terms found.");
        return Ok(());
    }

    let choice = Select::new("Select terms to view:", all).prompt()?;

    // Terms Display impl already shows id/name/due
    println!("{}", choice);
    Ok(())
}

async fn add(db: &SqliteStorage) -> Result<()> {
    let name    = Text::new("Name:").prompt()?;
    let due_str = Text::new("Due (days):").with_default("30").prompt()?;
    let due: i64 = due_str.trim().parse()
        .map_err(|_| anyhow!("'{}' is not a valid number of days.", due_str))?;

    let id = db.create_terms(CreateTerms { name, due }).await?;
    println!("Created terms #{}.", id.0);
    Ok(())
}

async fn update(id: TermsId, db: &SqliteStorage) -> Result<()> {
    let existing = db
        .get_terms(id)
        .await?
        .ok_or_else(|| anyhow!("Terms #{} not found.", id.0))?;

    println!("Current: {}\n---", existing);

    let new_name = {
        let input = Text::new("Name:").with_default(&existing.name).prompt()?;
        if input == existing.name { None } else { Some(input) }
    };

    let new_due = {
        let input = Text::new("Due (days):")
            .with_default(&existing.due.to_string())
            .prompt()?;
        let parsed: i64 = input.trim().parse()
            .map_err(|_| anyhow!("'{}' is not a valid number of days.", input))?;
        if parsed == existing.due { None } else { Some(parsed) }
    };

    db.update_terms(id, UpdateTerms { name: new_name, due: new_due }).await?;
    println!("Updated terms #{}.", id.0);
    Ok(())
}

async fn delete(id: TermsId, db: &SqliteStorage) -> Result<()> {
    let existing = db
        .get_terms(id)
        .await?
        .ok_or_else(|| anyhow!("Terms #{} not found.", id.0))?;

    let confirmed = Confirm::new(&format!(
        "Delete terms '{}' (#{})? This cannot be undone.",
        existing.name, id.0
    ))
    .with_default(false)
    .prompt()?;

    if confirmed {
        db.delete_terms(id).await?;
        println!("Deleted terms #{}.", id.0);
    } else {
        println!("Cancelled.");
    }
    Ok(())
}
