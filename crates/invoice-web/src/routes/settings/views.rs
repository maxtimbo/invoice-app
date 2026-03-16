use std::sync::Arc;
use axum::{extract::{Query, State}, response::{Html, IntoResponse}};
use tera::Context;
use invoice_app::ports::repos::config_repo::ConfigRepo;
use crate::state::AppState;
use std::collections::HashMap;

type S = Arc<AppState>;

pub async fn email_form(
    State(s): State<S>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {

    let config = s.db.get_config().await.unwrap();
    let mut ctx = Context::new();
    ctx.insert("config", &config);
    ctx.insert("test", &params.get("test").map(|s| s.as_str()).unwrap_or(""));
    ctx.insert("test_msg", &params.get("msg").map(|s| s.replace('+', " ")));
    Html(s.tera.render("settings/email.html", &ctx).unwrap())
}
