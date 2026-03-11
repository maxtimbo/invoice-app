use anyhow::{anyhow, Result};
use clap::{Args, Subcommand};
use inquire::{Confirm, Select, Text};

use invoice_app::ports::repos::client_repo::{ClientRepo, CreateClient, UpdateClient};
use invoice_core::models::ids::ClientId;
use invoice_storage::sqlite::SqliteStorage;
use invoice_cli::{prompt_contact, prompt_id};

#[derive(Args)]
pub struct ClientArgs {
    #[command(subcommand)]
    pub command: Option<ClientCommand>,
}

#[derive(Subcommand)]
pub enum ClientCommand {
    /// List all clients and view details
    List,
    /// Add a new client
    Add,
    /// Update an existing client
    Update { id: i64 },
    /// Delete a client
    Delete { id: i64 },
}

pub async fn run(args: ClientArgs, db: &SqliteStorage) -> Result<()> {
    match args.command {
        Some(ClientCommand::List)            => list(db).await,
        Some(ClientCommand::Add)             => add(db).await,
        Some(ClientCommand::Update { id })   => update(ClientId(id), db).await,
        Some(ClientCommand::Delete { id })   => delete(ClientId(id), db).await,
        None => interactive(db).await,
    }
}

pub async fn interactive(db: &SqliteStorage) -> Result<()> {
    let choice = Select::new(
        "Client →",
        vec!["List", "Add", "Update", "Delete", "Back"],
    )
    .prompt()?;

    match choice {
        "List"   => list(db).await,
        "Add"    => add(db).await,
        "Update" => update(ClientId(prompt_id("Client ID:")?), db).await,
        "Delete" => delete(ClientId(prompt_id("Client ID:")?), db).await,
        _        => Ok(()),
    }
}

async fn list(db: &SqliteStorage) -> Result<()> {
    let all = db.list_client().await?;
    if all.is_empty() {
        println!("No clients found.");
        return Ok(());
    }

    let choice = Select::new("Select a client to view:", all).prompt()?;
    println!("{}", choice);
    Ok(())
}

async fn add(db: &SqliteStorage) -> Result<()> {
    let name    = Text::new("Name:").prompt()?;
    let contact = prompt_contact(None)?;

    let id = db.create_client(CreateClient { name, contact }).await?;
    println!("Created client #{}.", id.0);
    Ok(())
}

async fn update(id: ClientId, db: &SqliteStorage) -> Result<()> {
    let existing = db
        .get_client(id)
        .await?
        .ok_or_else(|| anyhow!("Client #{} not found.", id.0))?;

    println!("Current:\n{}\n---", existing);

    let new_name = {
        let input = Text::new("Name:").with_default(&existing.name).prompt()?;
        if input == existing.name { None } else { Some(input) }
    };

    let new_contact = if Confirm::new("Update contact info?").with_default(false).prompt()? {
        Some(prompt_contact(Some(&existing.contact))?)
    } else {
        None
    };

    db.update_client(id, UpdateClient { name: new_name, contact: new_contact }).await?;
    println!("Updated client #{}.", id.0);
    Ok(())
}

async fn delete(id: ClientId, db: &SqliteStorage) -> Result<()> {
    let existing = db
        .get_client(id)
        .await?
        .ok_or_else(|| anyhow!("Client #{} not found.", id.0))?;

    let confirmed = Confirm::new(&format!(
        "Delete client '{}' (#{})? This cannot be undone.",
        existing.name, id.0
    ))
    .with_default(false)
    .prompt()?;

    if confirmed {
        db.delete_client(id).await?;
        println!("Deleted client #{}.", id.0);
    } else {
        println!("Cancelled.");
    }
    Ok(())
}
