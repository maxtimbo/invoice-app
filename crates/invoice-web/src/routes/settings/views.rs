use std::sync::Arc;
use axum::{extract::State, response::{Html, IntoResponse}};
use tera::Context;
use invoice_app::ports::repos::config_repo::ConfigRepo;
use crate::state::AppState;

type S = Arc<AppState>;

pub async fn email_form(State(s): State<S>) -> impl IntoResponse {
    let config = s.db.get_config().await.unwrap();
    let mut ctx = Context::new();
    ctx.insert("config", &config);
    Html(s.tera.render("settings/email.html", &ctx).unwrap())
}
