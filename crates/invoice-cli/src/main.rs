use anyhow::Result;

use invoice_app::commands::paths::Paths;
use invoice_storage::sqlite::SqliteStorage;
use invoice_app::ports::repos::invoice_repo::InvoiceRepo;

#[tokio::main]
async fn main() -> Result<()> {
    let paths = Paths::init()?;
    let db = SqliteStorage::connect(paths.db.to_str().unwrap()).await?;
    db.migrate().await?;

    let invoices = db.list_invoice_summary().await?;
    for i in invoices {
        println!("{}: {}\nissued: {}\ndue date: {}\ntotal: {}\nstatus: {}\n\n", i.id, i.client_name, i.issued, i.due, i.total, i.status);
    }

    //let renderer = TemplateEngine::new(&paths.templates)?;
    //Cli::to_cmd(&mut db, &renderer)?;
    Ok(())
}
