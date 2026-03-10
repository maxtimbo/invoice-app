use anyhow::{anyhow, Context, Result};
use lettre::{
    message::{header::ContentType, Attachment, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

use invoice_core::models::{config::Config, invoice::Invoice};

pub struct EmailService;

impl EmailService {
    pub async fn send(
        config: &Config,
        invoice: &Invoice,
        html: String,
        pdf: Vec<u8>,
        pdf_filename: String,
    ) -> Result<()> {
        let to_email = invoice.template.client.contact.email
            .as_deref()
            .ok_or_else(|| anyhow!("Client has no email address - cannot send invoice"))?;

        let from = format!("{} <{}>", config.fromname, config.username)
            .parse()
            .context("Invalid from address in email config")?;

        let to = to_email
            .parse()
            .with_context(|| format!("Invalid client email address: {to_email}"))?;

        // tweak this
        let subject = format!(
            "Invoice ${:04} from {}",
            invoice.id, config.fromname
        );

        let attachment = Attachment::new(pdf_filename).body(
            pdf,
            ContentType::parse("application/pdf").unwrap(),
        );

        let email = Message::builder()
            .from(from)
            .to(to)
            .subject(subject)
            .multipart(
                MultiPart::mixed()
                .singlepart(
                    SinglePart::builder()
                    .header(ContentType::TEXT_HTML)
                    .body(html),
                )
                .singlepart(attachment),
            )
            .context("Failed to build email message")?;

        let creds = Credentials::new(config.username.clone(), config.password.clone());

        let transport: AsyncSmtpTransport<Tokio1Executor> = if config.tls {
            AsyncSmtpTransport::<Tokio1Executor>::relay(&config.smtp_server)
                .context("Failed to build SMTP transport")?
                .port(config.port)
                .credentials(creds)
                .build()
        } else {
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&config.smtp_server)
                .port(config.port)
                .credentials(creds)
                .build()
        };

        transport
            .send(email)
            .await
            .context("Failed to send email")?;

        Ok(())
    }
}
