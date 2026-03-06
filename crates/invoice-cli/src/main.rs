use anyhow::Result;

use invoice_app::commands::paths::Paths;
use invoice_storage::sqlite::SqliteStorage;
use invoice_app::ports::repos::invoice_repo::InvoiceRepo;

#[tokio::main]
async fn main() -> Result<()> {
    let paths = Paths::init()?;
    let db = SqliteStorage::connect(paths.db.to_str().unwrap()).await?;
    let invoices = db.list_invoice_summary().await?;
    for i in invoices {
        println!("{}: {}\n{}\n{}", i.id, i.issued, i.client_name, i.total);
    }
    //let renderer = TemplateEngine::new(&paths.templates)?;
    //Cli::to_cmd(&mut db, &renderer)?;
    Ok(())
}
