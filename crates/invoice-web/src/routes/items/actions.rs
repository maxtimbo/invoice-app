use std::sync::Arc;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Form, Json,
};
use serde::Serialize;
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

#[derive(Serialize)]
pub struct CreatedItem {
    id: i64,
    name: String,
    rate: String,
}

/// JSON variant of `create` used by the quick-add modal on invoice pages.
pub async fn quick_create(State(s): State<S>, Form(input): Form<ItemForm>) -> impl IntoResponse {
    let rate = match input.parse_rate() {
        Ok(r) => r,
        Err(e) => return (StatusCode::BAD_REQUEST, e).into_response(),
    };
    let name = input.name.trim().to_string();
    if name.is_empty() {
        return (StatusCode::BAD_REQUEST, "Name is required").into_response();
    }
    let rate_str = format!("{:.2}", rate.inner());
    match s.db.create_item(CreateItem { name: name.clone(), rate }).await {
        Ok(id) => Json(CreatedItem { id: id.0, name, rate: rate_str }).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
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
