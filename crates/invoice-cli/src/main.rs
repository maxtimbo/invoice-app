use anyhow::Result;

use invoice_app::commands::paths::Paths;
use invoice_storage::sqlite::SqliteStorage;
use invoice_app::ports::repos::client_repo::ClientRepo;

#[tokio::main]
async fn main() -> Result<()> {
    let paths = Paths::init()?;
    let db = SqliteStorage::connect(paths.db.to_str().unwrap()).await?;
    let items = db.list_client().await?;
    for i in items {
        println!("{}: {}", i.id, i.name);
    }
    //let renderer = TemplateEngine::new(&paths.templates)?;
    //Cli::to_cmd(&mut db, &renderer)?;
    Ok(())
}
