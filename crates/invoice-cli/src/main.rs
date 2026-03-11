use anyhow::Result;
use clap::Parser;

use invoice_app::commands::paths::Paths;
use invoice_storage::sqlite::SqliteStorage;
use invoice_cli::is_cancelled;

mod interactive;
mod cli;

#[derive(Parser)]
#[command(name = "invoice", about = "Invoice management CLI", version)]
struct Cli {
    #[command(subcommand)]
    command: Option<cli::Commands>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let paths = Paths::init()?;
    let db = SqliteStorage::connect(paths.db.to_str().unwrap()).await?;
    db.migrate().await?;

    let cli = Cli::parse();

    let result = match cli.command {
        Some(cmd) => cli::dispatch(cmd, &db).await,
        None => interactive::run(&db).await,
    };

    match result {
        Err(e) if is_cancelled(&e) => Ok(()),
        other => other,
    }
}

