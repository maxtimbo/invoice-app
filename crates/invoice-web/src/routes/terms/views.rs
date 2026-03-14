use std::sync::Arc;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use tera::Context;
use invoice_app::ports::repos::terms_repo::TermsRepo;
use invoice_core::models::ids::TermsId;
use crate::state::AppState;

type S = Arc<AppState>;

pub async fn list(State(s): State<S>) -> impl IntoResponse {
    let terms = s.db.list_terms().await.unwrap();
    let mut ctx = Context::new();
    ctx.insert("terms", &terms);
    Html(s.tera.render("terms/list.html", &ctx).unwrap())
}

pub async fn new_form(State(s): State<S>) -> impl IntoResponse {
    Html(s.tera.render("terms/form.html", &Context::new()).unwrap())
}

pub async fn edit_form(State(s): State<S>, Path(id): Path<i64>) -> impl IntoResponse {
    let terms = s.db.get_terms(TermsId(id)).await.unwrap();
    let mut ctx = Context::new();
    ctx.insert("terms", &terms);
    Html(s.tera.render("terms/form.html", &ctx).unwrap())
}
