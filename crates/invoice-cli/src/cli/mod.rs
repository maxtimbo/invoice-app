pub mod client;
pub mod company;
pub mod invoice;
pub mod items;
pub mod methods;
pub mod template;
pub mod terms;
pub mod email;

use anyhow::Result;
use clap::Subcommand;

use invoice_storage::sqlite::SqliteStorage;

#[derive(Subcommand)]
pub enum Commands {
    /// manage companies
    Company(company::CompanyArgs),
    /// manage clients
    Client(client::ClientArgs),
    /// manage line items
    Items(items::ItemsArgs),
    /// manage payment terms
    Terms(terms::TermsArgs),
    /// manage payment methods
    Methods(methods::MethodsArgs),
    /// manage invoice templates
    Template(template::TemplateArgs),
    /// manage invoices
    Invoice(invoice::InvoiceArgs),
    /// send invoices and manage email config
    Email(email::EmailArgs),
}

pub async fn dispatch(cmd: Commands, db: &SqliteStorage) -> Result<()> {
    match cmd {
        Commands::Company(args)     => company::run(args, db).await,
        Commands::Client(args)      => client::run(args, db).await,
        Commands::Items(args)       => items::run(args, db).await,
        Commands::Terms(args)       => terms::run(args, db).await,
        Commands::Methods(args)     => methods::run(args, db).await,
        Commands::Template(args)    => template::run(args, db).await,
        Commands::Invoice(args)     => invoice::run(args, db).await,
        Commands::Email(args)       => email::run(db, args).await,
    }
}

