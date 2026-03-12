use anyhow::{anyhow, Result};
use clap::{Args, Subcommand};

use invoice_app::{
    ports::repos::{
        config_repo::{ConfigRepo, UpsertConfig},
        invoice_repo::InvoiceRepo,
    },
    services::email_service::EmailService,
};

use invoice_core::models::ids::InvoiceId;
use invoice_cli::render::{render_pdf, render_html};
use invoice_cli::prompt_optional;

#[derive(Debug, Args)]
pub struct EmailArgs {
    #[command(subcommand)]
    pub command: Option<EmailCommand>,
}

#[derive(Debug, Subcommand)]
pub enum EmailCommand {
    /// Send and invoice by email
    Send { id: i64 },
    /// show current smtp config
    ShowConfig,
    /// set smtp config creates or replace
    SetConfig,
    /// send a test email to verify settings
    TestConfig,
}

pub async fn run<R>(repo: &R, args: EmailArgs) -> Result<()>
where
    R: InvoiceRepo + ConfigRepo,
{
    match args.command {
        Some(EmailCommand::Send { id })     => send(repo, id).await,
        Some(EmailCommand::ShowConfig)      => show_config(repo).await,
        Some(EmailCommand::SetConfig)       => set_config(repo).await,
        Some(EmailCommand::TestConfig)      => test_config(repo).await,
        None => interactive(repo).await,
    }
}

pub async fn test_config<R: ConfigRepo>(repo: &R) -> Result<()> {
    let config = repo.get_config().await?
        .ok_or_else(|| anyhow!("email not configured"))?;
    EmailService::test_config(&config).await
}



pub async fn send<R: InvoiceRepo + ConfigRepo>(repo: &R, id: i64) -> Result<()> {
    let config = repo
        .get_config()
        .await?
        .ok_or_else(|| anyhow!("email not configured."))?;

    let invoice = repo
        .get_invoice(InvoiceId(id))
        .await?
        .ok_or_else(|| anyhow!("invoice {} not found", id))?;

    let html = render_html(&invoice)?;
    let pdf = render_pdf(&invoice)?;
    let filename = format!("invoice-{:04}.pdf", invoice.id);

    EmailService::send(&config, &invoice, html, pdf, filename).await?;

    println!("invoice {} sent to {}",
        id.to_string(), invoice.template.client.contact.email.as_deref().unwrap_or("(unknown)"));
    Ok(())
}

async fn show_config<R: ConfigRepo>(repo: &R) -> Result<()> {
    match repo.get_config().await? {
        None => println!("email not configured"),
        Some(c) => println!("{c}"),
    }
    Ok(())
}

async fn set_config<R: ConfigRepo>(repo: &R) -> Result<()> {
    use inquire::{Confirm, Text};

    let current     = repo.get_config().await?;
    let c_server    = current.as_ref().map(|c| c.smtp_server.as_str()).unwrap_or("");
    let c_port      = current.as_ref().map(|c| c.port.to_string()).unwrap_or_else(|| "587".into());
    let c_tls       = current.as_ref().map(|c| c.tls).unwrap_or(true);
    let c_username  = current.as_ref().map(|c| c.username.as_str()).unwrap_or("");
    let c_fromname  = current.as_ref().map(|c| c.fromname.as_str()).unwrap_or("");

    let smtp_server = Text::new("SMTP server:").with_default(c_server).prompt()?;
    let port_str    = Text::new("Port:").with_default(&c_port).prompt()?;
    let port: u16   = port_str.parse().map_err(|_| anyhow!("invalid port number"))?;
    let tls         = Confirm::new("Use TLS?").with_default(c_tls).prompt()?;
    let username    = Text::new("Username:").with_default(c_username).prompt()?;
    let password    = inquire::Password::new("password (leave blank to keep current):")
        .without_confirmation()
        .prompt()?;
    let fromname    = Text::new("From display name:").with_default(c_fromname).prompt()?;

    let password    = if password.is_empty() {
        current
            .as_ref()
            .map(|c| c.password.clone())
            .ok_or_else(|| anyhow!("password required"))?
    } else {
        password
    };

    let test_recipient = {
        let current = current.as_ref()
            .and_then(|c| c.test_recipient.as_deref())
            .unwrap_or("");
        prompt_optional("Test recipient email:", current)?
    };

    repo.upsert_config(UpsertConfig { smtp_server, port, tls, username, password, fromname, test_recipient })
        .await?;

    println!("Email config saved");
    Ok(())
}

pub async fn interactive<R: InvoiceRepo + ConfigRepo>(repo: &R) -> Result<()> {
    use inquire::Select;

    let options = vec!["Send invoice", "Show config", "Set config", "Test config", "Back"];
    match Select::new("Email:", options).prompt()? {
        "Send invoice" => {
            let id_str = inquire::Text::new("Invoice ID:").prompt()?;
            let id: i64 = id_str.parse().map_err(|_| anyhow!("Invalid ID"))?;
            send(repo, id).await
        }
        "Show config" => show_config(repo).await,
        "Set config" => set_config(repo).await,
        "Test config" => test_config(repo).await,
        _ => Ok(()),
    }
}
