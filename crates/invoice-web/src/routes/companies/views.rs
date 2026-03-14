use std::sync::Arc;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use tera::Context;
use invoice_app::ports::repos::company_repo::CompanyRepo;
use invoice_core::models::ids::CompanyId;
use crate::state::AppState;
use super::CompanyView;

type S = Arc<AppState>;

pub async fn list(State(s): State<S>) -> impl IntoResponse {
    let companies = s.db.list_company().await.unwrap();
    let mut ctx = Context::new();
    ctx.insert("companies", &companies);
    Html(s.tera.render("companies/list.html", &ctx).unwrap())
}

pub async fn new_form(State(s): State<S>) -> impl IntoResponse {
    Html(s.tera.render("companies/form.html", &Context::new()).unwrap())
}

pub async fn edit_form(State(s): State<S>, Path(id): Path<i64>) -> impl IntoResponse {
    let company = s.db.get_company(CompanyId(id)).await.unwrap();
    let mut ctx = Context::new();
    ctx.insert("company", &company.map(CompanyView::from));
    Html(s.tera.render("companies/form.html", &ctx).unwrap())
}
