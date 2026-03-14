use std::sync::Arc;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use tera::Context;
use invoice_core::models::ids::ItemId;
use invoice_app::ports::repos::item_repo::ItemRepo;
use crate::state::AppState;

type S = Arc<AppState>;

pub async fn list(State(s): State<S>) -> impl IntoResponse {
    let items = s.db.list_item().await.unwrap();
    let mut ctx = Context::new();
    ctx.insert("items", &items);
    Html(s.tera.render("items/list.html", &ctx).unwrap())
}

pub async fn new_form(State(s): State<S>) -> impl IntoResponse {
    Html(s.tera.render("items/form.html", &Context::new()).unwrap())
}

pub async fn edit_form(State(s): State<S>, Path(id): Path<i64>) -> impl IntoResponse {
    let item = s.db.get_item(ItemId(id)).await.unwrap();
    let mut ctx = Context::new();
    ctx.insert("item", &item);
    Html(s.tera.render("items/form.html", &ctx).unwrap())
}
