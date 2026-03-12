use anyhow::{anyhow, Result};
use clap::{Args, Subcommand};
use inquire::{Confirm, Select, Text};

use invoice_app::ports::repos::company_repo::{CompanyRepo, CreateCompany, UpdateCompany};
use invoice_core::models::{company::CompanyList, ids::CompanyId};
use invoice_storage::sqlite::SqliteStorage;
use invoice_cli::{prompt_image, prompt_id, prompt_contact, resolve_id};

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
    Update { id: Option<i64> },
    /// delete a company
    Delete { id: Option<i64> },
}

pub async fn run(args: CompanyArgs, db: &SqliteStorage) -> Result<()> {
    match args.command {
        Some(CompanyCommand::List)          => list(db).await,
        Some(CompanyCommand::Add)           => add(db).await,
        Some(CompanyCommand::Update { id }) => {
            let id = resolve_id!(id, db, list_company, CompanyList, CompanyId,
                "No companies found", "Select company:");
            update(id, db).await
        }
        Some(CompanyCommand::Delete { id }) => {
            let id = resolve_id!(id, db, list_company, CompanyList, CompanyId,
                "No companies found", "Select company:");
            delete(id, db).await
        }
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

async fn add(db: &SqliteStorage) -> Result<()> {
    let name    = Text::new("Name:").prompt()?;
    let logo    = prompt_image("Logo path (leave blank to skip):")?;
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
        prompt_image("New logo path (leave blank to clear):")?
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

