use std::sync::Arc;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use tera::Context;
use invoice_app::ports::repos::client_repo::ClientRepo;
use invoice_core::models::ids::ClientId;
use crate::state::AppState;

type S = Arc<AppState>;

pub async fn list(State(s): State<S>) -> impl IntoResponse {
    let clients = s.db.list_client().await.unwrap();
    let mut ctx = Context::new();
    ctx.insert("clients", &clients);
    Html(s.tera.render("clients/list.html", &ctx).unwrap())
}

pub async fn new_form(State(s): State<S>) -> impl IntoResponse {
    Html(s.tera.render("clients/form.html", &Context::new()).unwrap())
}

pub async fn edit_form(State(s): State<S>, Path(id): Path<i64>) -> impl IntoResponse {
    let client = s.db.get_client(ClientId(id)).await.unwrap();
    let mut ctx = Context::new();
    ctx.insert("client", &client);
    Html(s.tera.render("clients/form.html", &ctx).unwrap())
}
