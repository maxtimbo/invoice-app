use anyhow::Result;
use clap::{Args, Subcommand};
use invoice_storage::sqlite::SqliteStprage;

#[derive(Args)]
pub struct ClientArgs {
    #[command(subcommand)]
    pub command: Option<ClientCommand>,
}

#[derive(Subcommand)]
pub enum ClientCommand {
    /// list all clients
    List,
    /// add a new client
    Add,
    /// update an existing client
    Update { id: i64 },
    /// delete a client
    Delete { id: i64 },
}

pub async fn run(args: ClientArgs, db: &SqliteStorage) -> Result<()> {
    println!("Client management not yet implemented.");
    Ok(())
}

pub async fn interactive(db: &SqliteStorage) -> Result<()> {
    println!("Client management not yet implemented.");
    Ok(())
}
