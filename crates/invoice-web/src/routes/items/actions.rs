use std::sync::Arc;
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
    Form,
};
use invoice_core::models::ids::ItemId;
use invoice_app::ports::repos::item_repo::{ItemRepo, CreateItem, UpdateItem};
use crate::state::AppState;
use super::ItemForm;

type S = Arc<AppState>;

pub async fn create(State(s): State<S>, Form(input): Form<ItemForm>) -> impl IntoResponse {
    let rate = input.parse_rate().unwrap();
    s.db.create_item(CreateItem {
        name: input.name,
        rate,
    }).await.unwrap();
    Redirect::to("/items")
}

pub async fn update(
    State(s): State<S>,
    Path(id): Path<i64>,
    Form(input): Form<ItemForm>,
) -> impl IntoResponse {
    let rate = input.parse_rate().unwrap();
    s.db.update_item(ItemId(id), UpdateItem {
        name: Some(input.name),
        rate: Some(rate),
    }).await.unwrap();
    Redirect::to("/items")
}

pub async fn delete(State(s): State<S>, Path(id): Path<i64>) -> impl IntoResponse {
    s.db.delete_item(ItemId(id)).await.unwrap();
    Redirect::to("/items")
}
