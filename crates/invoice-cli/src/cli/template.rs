use anyhow::{anyhow, Result};
use clap::{Args, Subcommand};
use inquire::{Confirm, MultiSelect, Select, Text};

use invoice_app::ports::repos::{
    client_repo::ClientRepo,
    company_repo::CompanyRepo,
    method_repo::MethodRepo,
    template_repo::{CreateTemplate, TemplateRepo, UpdateTemplate},
    terms_repo::TermsRepo,
};
use invoice_core::models::ids::TemplateId;
use invoice_storage::sqlite::SqliteStorage;
use invoice_cli::prompt_id;

#[derive(Args)]
pub struct TemplateArgs {
    #[command(subcommand)]
    pub command: Option<TemplateCommand>,
}

#[derive(Subcommand)]
pub enum TemplateCommand {
    /// List all templates and view details
    List,
    /// Add a new template
    Add,
    /// Update an existing template
    Update { id: i64 },
    /// Delete a template
    Delete { id: i64 },
}

pub async fn run(args: TemplateArgs, db: &SqliteStorage) -> Result<()> {
    match args.command {
        Some(TemplateCommand::List)            => list(db).await,
        Some(TemplateCommand::Add)             => add(db).await,
        Some(TemplateCommand::Update { id })   => update(TemplateId(id), db).await,
        Some(TemplateCommand::Delete { id })   => delete(TemplateId(id), db).await,
        None => interactive(db).await,
    }
}

pub async fn interactive(db: &SqliteStorage) -> Result<()> {
    let choice = Select::new(
        "Templates →",
        vec!["List", "Add", "Update", "Delete", "Back"],
    )
    .prompt()?;

    match choice {
        "List"   => list(db).await,
        "Add"    => add(db).await,
        "Update" => update(TemplateId(prompt_id("Template ID:")?), db).await,
        "Delete" => delete(TemplateId(prompt_id("Template ID:")?), db).await,
        _        => Ok(()),
    }
}

async fn list(db: &SqliteStorage) -> Result<()> {
    let all = db.list_template().await?;
    if all.is_empty() {
        println!("No templates found.");
        return Ok(());
    }

    let choice = Select::new("Select a template to view:", all).prompt()?;
    println!("{}", choice);
    Ok(())
}

async fn add(db: &SqliteStorage) -> Result<()> {
    let name    = Text::new("Name:").prompt()?;
    let company = select_company(db).await?;
    let client  = select_client(db).await?;
    let terms   = select_terms(db).await?;
    let methods = select_methods(db).await?;

    let id = db.create_template(CreateTemplate {
        name,
        company: company.id,
        client:  client.id,
        terms:   terms.id,
        method:  methods.into_iter().map(|m| m.id).collect(),
    })
    .await?;

    println!("Created template #{}.", id.0);
    Ok(())
}

async fn update(id: TemplateId, db: &SqliteStorage) -> Result<()> {
    let existing = db
        .get_template(id)
        .await?
        .ok_or_else(|| anyhow!("Template #{} not found.", id.0))?;

    println!("Current:\n{}\n---", existing);

    let new_name = {
        let input = Text::new("Name:").with_default(&existing.name).prompt()?;
        if input == existing.name { None } else { Some(input) }
    };

    let new_company = if Confirm::new("Update company?").with_default(false).prompt()? {
        Some(select_company(db).await?.id)
    } else {
        None
    };

    let new_client = if Confirm::new("Update client?").with_default(false).prompt()? {
        Some(select_client(db).await?.id)
    } else {
        None
    };

    let new_terms = if Confirm::new("Update terms?").with_default(false).prompt()? {
        Some(select_terms(db).await?.id)
    } else {
        None
    };

    let new_methods = if Confirm::new("Update payment methods?").with_default(false).prompt()? {
        Some(select_methods(db).await?.into_iter().map(|m| m.id).collect())
    } else {
        None
    };

    db.update_template(id, UpdateTemplate {
        name:    new_name,
        company: new_company,
        client:  new_client,
        terms:   new_terms,
        method:  new_methods,
    })
    .await?;

    println!("Updated template #{}.", id.0);
    Ok(())
}

async fn delete(id: TemplateId, db: &SqliteStorage) -> Result<()> {
    let existing = db
        .get_template(id)
        .await?
        .ok_or_else(|| anyhow!("Template #{} not found.", id.0))?;

    let confirmed = Confirm::new(&format!(
        "Delete template '{}' (#{})? This cannot be undone.",
        existing.name, id.0
    ))
    .with_default(false)
    .prompt()?;

    if confirmed {
        db.delete_template(id).await?;
        println!("Deleted template #{}.", id.0);
    } else {
        println!("Cancelled.");
    }
    Ok(())
}

// ── Selectors ────────────────────────────────────────────────────────────────

async fn select_company(db: &SqliteStorage) -> Result<invoice_core::models::company::CompanyList> {
    let all = db.list_company().await?;
    if all.is_empty() {
        return Err(anyhow!("No companies found. Add one first."));
    }
    Ok(Select::new("Company:", all).prompt()?)
}

async fn select_client(db: &SqliteStorage) -> Result<invoice_core::models::client::ClientList> {
    let all = db.list_client().await?;
    if all.is_empty() {
        return Err(anyhow!("No clients found. Add one first."));
    }
    Ok(Select::new("Client:", all).prompt()?)
}

async fn select_terms(db: &SqliteStorage) -> Result<invoice_core::models::terms::Terms> {
    let all = db.list_terms().await?;
    if all.is_empty() {
        return Err(anyhow!("No terms found. Add one first."));
    }
    Ok(Select::new("Terms:", all).prompt()?)
}

async fn select_methods(db: &SqliteStorage) -> Result<Vec<invoice_core::models::method::Method>> {
    let all = db.list_method().await?;
    if all.is_empty() {
        return Err(anyhow!("No payment methods found. Add one first."));
    }
    Ok(MultiSelect::new("Payment methods (space to select):", all).prompt()?)
}
