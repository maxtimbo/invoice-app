use anyhow::{anyhow, Context, Result};
use lettre::{
    message::{header::ContentType, Attachment, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

use invoice_core::models::{config::Config, invoice::Invoice};

pub struct EmailService;

impl EmailService {
    pub async fn test_config(config: &Config) -> Result<()> {
        let to_email = config.test_recipient
            .as_deref()
            .ok_or_else(|| anyhow!("No test recipient configured"))?;

        let from = format!("\"{}\" <{}>", config.fromname, config.username)
            .parse()
            .context("invalid from address")?;

        let to = to_email.parse()
            .with_context(|| format!("invalid test recipient address: {to_email}"))?;

        let email = Message::builder()
            .from(from)
            .to(to)
            .subject("invoice-cli: test email")
            .body("test email".to_string())
            .context("failed to build test email")?;

        Self::build_transport(config)?
            .send(email)
            .await
            .context("failed to send email")?;
        println!("test complete");
        Ok(())
    }
    fn build_transport(config: &Config) -> Result<AsyncSmtpTransport<Tokio1Executor>> {
        let creds = Credentials::new(config.username.clone(), config.password.clone());

        let transport: AsyncSmtpTransport<Tokio1Executor> = if config.tls {
            AsyncSmtpTransport::<Tokio1Executor>::relay(&config.smtp_server)
                .context("Failed to build SMTP transport")?
                .port(config.port)
                .credentials(creds)
                .build()
        } else {
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.smtp_server)
                .context("Failed to build SMTP transport")?
                .port(config.port)
                .credentials(creds)
                .build()
        };
        Ok(transport)
    }
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

        let from = format!("\"{}\" <{}>", config.fromname, config.username)
            .parse()
            .context("Invalid from address in email config")?;

        let subject = invoice.email_subject();

        let attachment = Attachment::new(pdf_filename).body(
            pdf,
            ContentType::parse("application/pdf").unwrap(),
        );

        let email = {

            let mut builder = Message::builder().from(from);

            for addr in to_email.split(',').map(str::trim).filter(|s| !s.is_empty()) {
                let parsed = addr.parse()
                    .with_context(|| format!("invalid client email address: {addr}"))?;
                builder = builder.to(parsed);
            }
            builder
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
                .context("Failed to build email message")?
        };

        let transport = Self::build_transport(config)?;
        transport
            .send(email)
            .await
            .context("Failed to send email")?;

        Ok(())
    }
}
