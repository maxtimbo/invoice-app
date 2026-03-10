use anyhow::{anyhow, Result};
use clap::{Args, Subcommand};
use inquire::{Confirm, Select, Text};
use std::path::PathBuf;

use invoice_app::ports::repos::company_repo::{CompanyRepo, CreateCompany, UpdateCompany};
use invoice_core::models::{contact::Contact, ids::CompanyId};
use invoice_storage::sqlite::SqliteStorage;
use invoice_cli::prompt_optional;

#[derive(Args)]
pub struct CompanyArgs {
    #[command(subcommand)]
    pub command: Option<CompanyCommand>,
}

#[derive(Subcommand)]
pub enum CompanyCommand {
    /// list all companies
    List,
    /// add a new company
    Add,
    /// update an existing company
    Update { id: i64 },
    /// delete a company
    Delete { id: i64 },
}

pub async fn run(args: CompanyArgs, db: &SqliteStorage) -> Result<()> {
    match args.command {
        Some(CompanyCommand::List)          => list(db).await,
        Some(CompanyCommand::Add)           => add(db).await,
        Some(CompanyCommand::Update { id }) => update(CompanyId(id), db).await,
        Some(CompanyCommand::Delete { id }) => delete(CompanyId(id), db).await,
        None => interactive(db).await,
    }
}

pub async fn interactive(db: &SqliteStorage) -> Result<()> {
    let choice = Select::new(
        "Company ->",
        vec!["list", "add", "update", "delete", "back"],
    )
    .prompt()?;

    match choice {
        "list"      => list(db).await,
        "add"       => add(db).await,
        "update"    => update(CompanyId(prompt_id("company id:")?), db).await,
        "delete"    => delete(CompanyId(prompt_id("company id:")?), db).await,
        _           => Ok(()),
    }
}

async fn list(db: &SqliteStorage) -> Result<()> {
    let companies = db.list_company().await?;
    if companies.is_empty() {
        println!("No companies found.");
    } else {
        for c in &companies {
            println!("{}", c);
            println!("---");
        }
        println!("{} companies", companies.len());
    }
    Ok(())
}

async fn get(id: CompanyId, db: &SqliteStorage) -> Result<()> {
    match db.get_company(id).await? {
        Some(c) => println!("{}", c),
        None    => println!("Company {} not found", id.0),
    }
    Ok(())
}

async fn add(db: &SqliteStorage) -> Result<()> {
    let name    = Text::new("Name:").prompt()?;
    let logo    = prompt_logo("Logo path (leave blank to skip):")?;
    let contact = prompt_contact(None)?;

    let id = db.create_company(CreateCompany { name, logo, contact }).await?;
    println!("Created company {}", id.0);
    Ok(())
}

async fn update(id: CompanyId, db: &SqliteStorage) -> Result<()> {
    let existing = db
        .get_company(id)
        .await?
        .ok_or_else(|| anyhow!("Company {} not found", id.0))?;

    println!("{}", existing);

    let new_name = {
        let input = Text::new("Name:")
            .with_default(&existing.name)
            .prompt()?;
        if input == existing.name { None } else { Some(input) }
    };

    let new_logo = if Confirm::new("Update logo?").with_default(false).prompt()? {
        prompt_logo("New logo path (leave blank to clear):")?
    } else {
        existing.logo
    };

    let new_contact = if Confirm::new("Update contact info?")
        .with_default(false)
        .prompt()?
    {
        Some(prompt_contact(Some(&existing.contact))?)
    } else {
        None
    };

    let logo_patch = if Confirm::new("Update logo?").with_default(false).prompt().is_ok() {
        new_logo
    } else {
        None
    };

    db.update_company(
        id,
        UpdateCompany {
            name:       new_name,
            logo:       logo_patch,
            contact:    new_contact,
        },
    )
    .await?;

    println!("Updated company {}", id.0);
    Ok(())
}

async fn delete(id: CompanyId, db: &SqliteStorage) -> Result<()> {
    let existing = db
        .get_company(id)
        .await?
        .ok_or_else(|| anyhow!("Company {} not found", id.0))?;

    let confirmed = Confirm::new(&format!(
            "Delete company {}: {}? This cannot be undone",
            id.0, existing.name
        ))
        .with_default(false)
        .prompt()?;

    if confirmed {
        db.delete_company(id).await?;
        println!("Deleted company {}", id.0);
    } else {
        println!("cancelled");
    }
    Ok(())
}

/// Reads and validates a logo image. Returns None if the user skips.

/// Parse an i64 ID from an interactive prompt.
fn prompt_id(label: &str) -> Result<i64> {
    let s = Text::new(label).prompt()?;
    s.trim()
        .parse::<i64>()
        .map_err(|_| anyhow!("'{}' is not a valid ID.", s))
}
