use anyhow::Result;

use invoice_app::commands::paths::Paths;
use invoice_storage::db::InvoiceDB;

fn main() -> Result<()> {
    let paths = Paths::init()?;
    let mut db = InvoiceDB::open(paths.db, 2)?;
    let renderer = TemplateEngine::new(&paths.templates)?;
    Cli::to_cmd(&mut db, &renderer)?;
    Ok(())
}
