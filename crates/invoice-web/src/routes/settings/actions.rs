use std::sync::Arc;
use axum::{
    extract::State,
    response::{IntoResponse, Redirect, Html},
    Form,
};
use invoice_app::ports::repos::config_repo::{ConfigRepo, UpsertConfig};
use invoice_app::services::email_service::EmailService;
use crate::state::AppState;
use super::EmailConfigForm;

type S = Arc<AppState>;

pub async fn save_email_config(
    State(s): State<S>,
    Form(input): Form<EmailConfigForm>,
) -> impl IntoResponse {
    let port: u16 = input.port.parse().unwrap_or(587);
    let tls = input.tls.is_some();

    // Keep existing password if blank was submitted
    let password = match input.password.filter(|p| !p.trim().is_empty()) {
        Some(p) => p,
        None => s.db.get_config().await.unwrap()
            .map(|c| c.password)
            .unwrap_or_default(),
    };

    s.db.upsert_config(UpsertConfig {
        smtp_server: input.smtp_server,
        port,
        tls,
        username: input.username,
        password,
        fromname: input.fromname,
        test_recipient: input.test_recipient.filter(|s| !s.trim().is_empty()),
    }).await.unwrap();

    Redirect::to("/settings/email")
}

pub async fn test_email_config(State(s): State<S>) -> impl IntoResponse {
    let config = match s.db.get_config().await.unwrap() {
        Some(c) => c,
        None => return Html("<p>No email config found. <a href='/settings/email'>Configure it first.</a></p>".to_string()),
    };

    match EmailService::test_config(&config).await {
        Ok(_)  => Html("<p style='color:green'>✓ Test email sent successfully.</p> <a href='/settings/email'>Back</a>".to_string()),
        Err(e) => Html(format!("<p style='color:#cc4444'>✗ Failed: {e}</p> <a href='/settings/email'>Back</a>")),
    }
}
