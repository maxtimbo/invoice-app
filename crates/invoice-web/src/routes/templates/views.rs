use std::sync::Arc;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use tera::Context;
use invoice_app::ports::repos::template_repo::TemplateRepo;
use invoice_app::ports::repos::company_repo::CompanyRepo;
use invoice_app::ports::repos::client_repo::ClientRepo;
use invoice_app::ports::repos::terms_repo::TermsRepo;
use invoice_app::ports::repos::method_repo::MethodRepo;
use invoice_core::models::ids::TemplateId;
use crate::state::AppState;

type S = Arc<AppState>;

pub async fn list(State(s): State<S>) -> impl IntoResponse {
    let templates = s.db.list_template().await.unwrap();
    let mut ctx = Context::new();
    ctx.insert("templates", &templates);
    Html(s.tera.render("templates/list.html", &ctx).unwrap())
}

pub async fn new_form(State(s): State<S>) -> impl IntoResponse {
    let mut ctx = Context::new();
    populate_dropdowns(&s, &mut ctx).await;
    Html(s.tera.render("templates/form.html", &ctx).unwrap())
}

pub async fn edit_form(State(s): State<S>, Path(id): Path<i64>) -> impl IntoResponse {
    let template = s.db.get_template(TemplateId(id)).await.unwrap();
    let mut ctx = Context::new();
    ctx.insert("template", &template);
    populate_dropdowns(&s, &mut ctx).await;
    Html(s.tera.render("templates/form.html", &ctx).unwrap())
}

async fn populate_dropdowns(s: &AppState, ctx: &mut Context) {
    ctx.insert("companies", &s.db.list_company().await.unwrap());
    ctx.insert("clients",   &s.db.list_client().await.unwrap());
    ctx.insert("terms",     &s.db.list_terms().await.unwrap());
    ctx.insert("methods",   &s.db.list_method().await.unwrap());
}
