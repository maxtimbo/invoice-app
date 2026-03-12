use anyhow::Result;
use inquire::Select;

use invoice_storage::sqlite::SqliteStorage;

use crate::cli;

pub async fn run(db: &SqliteStorage) -> Result<()> {
    loop {
        let choice = Select::new(
            "What would you like to manage?",
            vec![
                "Company",
                "Client",
                "Items",
                "Terms",
                "Methods",
                "Templates",
                "Invoices",
                "Email",
                "Quit",
            ],
        )
        .prompt()?;

        match choice {
            "Company"   => cli::company::interactive(db).await?,
            "Client"    => cli::client::interactive(db).await?,
            "Items"     => cli::items::interactive(db).await?,
            "Terms"     => cli::terms::interactive(db).await?,
            "Methods"   => cli::methods::interactive(db).await?,
            "Templates" => cli::template::interactive(db).await?,
            "Invoices"  => cli::invoice::interactive(db).await?,
            "Email"     => cli::email::interactive(db).await?,
            "Quit"      => break,
            _           => {}
        }
    }

    Ok(())
}
